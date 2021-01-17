use serde_derive::{
  Deserialize,
  Serialize,
};

/// Represents an object with limited knowledge of it's dependencies.
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
  name: String,
  depends_on: Vec<String>
}

impl Item {
    /// Creates a new `Item` with no name and no dependencies.
    /// This is functionally identical to the value returned by `Default::default()`.
    pub fn new() -> Item {
        Default::default()
    }

    /// Creates a new `Item` with the given name, but no dependencies.
    pub fn with_name<T>(name: T) -> Item where T: Into<String> {
        Item {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Creates a new `Item` with the given name and dependencies.
    pub fn with_name_and_dependencies<T>(name: T, dependencies: Vec<String>) -> Item
    where T: Into<String> {
        Item {
            name: name.into(),
            depends_on: dependencies
        }
    }

    /// Get an immutable reference to the name of the `Item`.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Changes the name of the `Item` to the one given.
    pub fn set_name<T>(&mut self, new_name: T) where T: Into<String> {
        self.name = new_name.into();
    }

    /// Get an immutable reference to the list of dependencies of the `Item`.
    pub fn depends_on(&self) -> &Vec<String> {
        &self.depends_on
    }

    /// Adds a new dependency to the `Item`.
    /// Duplicate dependencies will be removed from the list.
    pub fn add_dependency<T>(&mut self, new_dep: T) where T: Into<String> {
        self.depends_on.push(new_dep.into());
        self.remove_duplicates();
    }

    /// Removes the given dependency from the `Item` if it exists.
    pub fn remove_dependency<T>(&mut self, dep: T) where T: Into<String> + Copy {
        self.depends_on.retain(|elem| elem.as_str() != dep.into().as_str());
    }

    /// Removes all the dependencies from the `Item`.
    pub fn clear_dependencies(&mut self) {
        self.depends_on.clear()
    }

    /// Adds multiple new dependencies to the `Item`.
    /// Duplicate dependencies will be removed from the list.
    pub fn add_dependencies(&mut self, deps: &Vec<String>) {
        self.depends_on.append(&mut deps.clone());
        self.remove_duplicates();
    }

    /// Removes multiple dependencies from the `Item` if they exist.
    pub fn remove_dependencies(&mut self, deps: &Vec<String>) {
        for dep in deps {
            self.remove_dependency(dep);
        }
    }

    /// Sets the dependency list of the `Item` to contain only the dependencies given
    /// Duplicate dependencies will be removed from the list.
    pub fn set_dependencies(&mut self, deps: &Vec<String>) {
        self.clear_dependencies();
        self.add_dependencies(deps);
    }

    fn remove_duplicates(&mut self) {
        self.depends_on.sort_unstable();
        self.depends_on.dedup();
    }
}

// TODO: Finish tests
#[cfg(test)]
mod tests {
    use crate::Item;

    #[test]
    fn get_function_test() {
        let mut item = Item::with_name("Test Item");
        assert_eq!(item.name(), "Test Item");

        item.add_dependency("Test Dependency");
        assert_eq!(item.depends_on().get(0).expect(""), "Test Dependency");
    }

    #[test]
    fn serialisation_test() {
        let mut item = Item::with_name_and_dependencies("Test Item", vec![String::from("Test Dependency 1"), String::from("Test Dependency 2")]);

        println!("{}", serde_json::to_string_pretty(&item).unwrap());
    }

    #[test]
    fn deduplication_test() {
        let mut item = Item::with_name_and_dependencies("Test Item", vec![String::from("Test Dependency 1"), String::from("Test Dependency 2")]);
        item.add_dependency("Test Dependency 1");

        assert_eq!(item.depends_on.len(), 2);
    }
}