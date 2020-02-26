/// Represents an object with limited knowledge of it's dependencies.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
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
  pub fn with_name(name: &String) -> Item {
    Item {
      name: String::from(name),
      ..Default::default()
    }
  }

  /// Creates a new `Item` with the given name and dependencies.
  pub fn with_name_and_dependencies(name: &String, dependencies: &Vec<String>) -> Item {
    Item {
      name: String::from(name),
      depends_on: dependencies.clone()
    }
  }
  
  /// Get an immutable reference to the name of the `Item`.
  pub fn name(&self) -> &String {
    &self.name
  }

  /// Changes the name of the `Item` to the one given.
  pub fn set_name(&mut self, new_name: &String) {
    self.name = String::from(new_name)
  }
  
  /// Get an immutable reference to the list of dependencies of the `Item`.
  pub fn depends_on(&self) -> &Vec<String> {
    &self.depends_on
  }

  /// Adds a new dependency to the `Item`.
  /// Duplicate dependencies will be removed from the list.
  pub fn add_dependency(&mut self, new_dep: &String) {
    self.depends_on.push(String::from(new_dep));
    self.remove_duplicates();
  }
  
  /// Removes the given dependency from the `Item` if it exists.
  pub fn remove_dependency(&mut self, dep: &String) {
    self.depends_on.retain(|elem| elem != dep);
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
