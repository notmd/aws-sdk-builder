use crate::error::BuildError;

#[derive(Debug, Clone, Default)]
pub struct Builder {
    pub(crate) services: Vec<ServiceSelection>,
}

/// Operation-name collections accepted by [`Builder::add`].
///
/// The array implementation keeps the required `add("s3", [])` call
/// unambiguous while the collection implementations support owned and borrowed
/// operation names without making consumers annotate an iterator item type.
pub trait OperationNames {
    fn into_operation_names(self) -> Vec<String>;
}

impl<const N: usize> OperationNames for [&'static str; N] {
    fn into_operation_names(self) -> Vec<String> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<T> OperationNames for Vec<T>
where
    T: Into<String>,
{
    fn into_operation_names(self) -> Vec<String> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<T> OperationNames for std::vec::IntoIter<T>
where
    T: Into<String>,
{
    fn into_operation_names(self) -> Vec<String> {
        self.map(Into::into).collect()
    }
}

impl<T> OperationNames for std::iter::Empty<T>
where
    T: Into<String>,
{
    fn into_operation_names(self) -> Vec<String> {
        self.map(Into::into).collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ServiceSelection {
    pub(crate) key: String,
    pub(crate) operations: Vec<String>,
    pub(crate) all_operations: bool,
}

impl Builder {
    /// Adds a service selection. An empty iterator selects every operation.
    pub fn add<O>(mut self, service: impl Into<String>, operations: O) -> Self
    where
        O: OperationNames,
    {
        let operations = operations.into_operation_names();
        self.services.push(ServiceSelection {
            key: service.into(),
            all_operations: operations.is_empty(),
            operations,
        });
        self
    }

    pub(crate) fn resolve(self) -> Result<Vec<ServiceSelection>, BuildError> {
        if self.services.is_empty() {
            return Err(BuildError::NoServices);
        }

        let mut merged = std::collections::BTreeMap::<String, ServiceSelection>::new();
        for selection in self.services {
            let entry = merged
                .entry(selection.key.clone())
                .or_insert_with(|| ServiceSelection {
                    key: selection.key.clone(),
                    operations: Vec::new(),
                    all_operations: false,
                });
            if selection.all_operations {
                entry.all_operations = true;
                entry.operations.clear();
            } else if !entry.all_operations {
                entry.operations.extend(selection.operations);
                entry.operations.sort();
                entry.operations.dedup();
            }
        }
        Ok(merged.into_values().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_entries_merge_deterministically() {
        let selections = Builder::default()
            .add("s3", ["PutObject", "GetObject", "GetObject"])
            .add("s3", ["PutObject"])
            .resolve()
            .unwrap();
        assert_eq!(selections[0].operations, ["GetObject", "PutObject"]);
    }

    #[test]
    fn all_operations_wins_over_narrower_entries() {
        let selections = Builder::default()
            .add("s3", ["GetObject"])
            .add("s3", std::iter::empty::<&str>())
            .add("s3", ["PutObject"])
            .resolve()
            .unwrap();
        assert!(selections[0].all_operations);
        assert!(selections[0].operations.is_empty());
    }

    #[test]
    fn an_untyped_empty_array_is_the_all_operations_form() {
        let selections = Builder::default().add("s3", []).resolve().unwrap();
        assert!(selections[0].all_operations);
    }

    #[test]
    fn owned_operation_names_use_the_same_public_api() {
        let selections = Builder::default()
            .add("s3", vec![String::from("GetObject")])
            .resolve()
            .unwrap();
        assert_eq!(selections[0].operations, ["GetObject"]);
    }
}
