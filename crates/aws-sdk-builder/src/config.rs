use crate::{error::BuildError, registry::ServiceSource};

#[derive(Debug, Clone, Default)]
pub struct Builder {
    pub(crate) services: Vec<ServiceSelection>,
}

/// Operation-name collections accepted by service builder compile calls.
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
    pub(crate) source: ServiceSource,
    pub(crate) operations: Vec<String>,
    pub(crate) all_operations: bool,
}

pub(crate) fn selection<O>(source: ServiceSource, operations: O) -> ServiceSelection
where
    O: OperationNames,
{
    let operations = operations.into_operation_names();
    let all_operations = operations.is_empty();
    ServiceSelection {
        source,
        operations,
        all_operations,
    }
}

impl Builder {
    /// Adds a service selection. An empty iterator selects every operation.
    pub fn add<O>(mut self, source: ServiceSource, operations: O) -> Self
    where
        O: OperationNames,
    {
        self.services.push(selection(source, operations));
        self
    }

    pub(crate) fn resolve(self) -> Result<Vec<ServiceSelection>, BuildError> {
        merge_selections(self.services)
    }
}

pub(crate) fn merge_selections(
    selections: Vec<ServiceSelection>,
) -> Result<Vec<ServiceSelection>, BuildError> {
    if selections.is_empty() {
        return Err(BuildError::NoServices);
    }

    let mut merged = std::collections::BTreeMap::<String, ServiceSelection>::new();
    for selection in selections {
        let key = selection.source.metadata.key.to_owned();
        let entry = merged.entry(key).or_insert_with(|| ServiceSelection {
            source: selection.source,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::ServiceMetadata;

    fn source() -> ServiceSource {
        ServiceSource::new(
            ServiceMetadata {
                key: "s3",
                filename: "model.json",
                module_name: "aws_sdk_s3",
                sdk_version: None,
            },
            br#"{"shapes":{}}"#,
        )
    }

    #[test]
    fn repeated_entries_merge_deterministically() {
        let selections = Builder::default()
            .add(source(), ["PutObject", "GetObject", "GetObject"])
            .add(source(), ["PutObject"])
            .resolve()
            .unwrap();
        assert_eq!(selections[0].operations, ["GetObject", "PutObject"]);
    }

    #[test]
    fn all_operations_wins_over_narrower_entries() {
        let selections = Builder::default()
            .add(source(), ["GetObject"])
            .add(source(), std::iter::empty::<&str>())
            .add(source(), ["PutObject"])
            .resolve()
            .unwrap();
        assert!(selections[0].all_operations);
        assert!(selections[0].operations.is_empty());
    }

    #[test]
    fn an_untyped_empty_array_is_the_all_operations_form() {
        let selections = Builder::default().add(source(), []).resolve().unwrap();
        assert!(selections[0].all_operations);
    }
}
