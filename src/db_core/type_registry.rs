use std::collections::HashMap;

struct TypeEntry {
    int_val: u32,
    string_val: String
}

type TypeRegistry = HashMap<u32, TypeEntry>;


fn mock_registry() -> TypeRegistry {
    let mut tr = TypeRegistry::new();
    tr.insert(0, TypeEntry{int_val: 0, string_val: "&str".to_string()});
    tr.insert(1, TypeEntry{int_val: 1, string_val: "String".to_string()});
    tr
}


fn get_registry<'a>() -> TypeRegistry {
    mock_registry()
}

fn get_string_val_from_int<'a>(int_val: u32) -> &'a str{
    "a"
}


#[test]
fn mock_registry_works() {
    let tr = mock_registry();
    assert_eq!(0,           tr.get(&0).unwrap().int_val);
    assert_eq!("&str",      tr.get(&0).unwrap().string_val);
    assert_eq!(1,           tr.get(&1).unwrap().int_val);
    assert_eq!("String",    tr.get(&1).unwrap().string_val);
}
