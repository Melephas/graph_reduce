# Design

``` rust
struct Item {
    name: String,
    dependencies: Vec<String>
}

impl Item {
    fn convert_to_dag() -> daggy::Dag;
}
```