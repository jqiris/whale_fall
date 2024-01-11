use super::do_def::*;
use super::type_def::*;
#[test]
fn type_def_use() {
    let fields = vec![
        Field {
            field: "ID".to_string(),
            gen_slice_func: true,
            type_: "int".to_string(),
            ..Default::default()
        },
        Field {
            field: "Name".to_string(),
            gen_slice_func: true,
            type_: "string".to_string(),
            ..Default::default()
        },
    ];
    let entity_type = EntityTypeMap {
        entity_name: "User".to_string(),
        entity_list_name: "UserList".to_string(),
        fields,
        ..Default::default()
    };
    let res = entity_type.execute();
    // assert!(res.is_ok());
    // assert!(res.unwrap().contains("func (e *User) String() string"));
    println!("{}", res.unwrap());
}

#[test]
fn do_def_use() {
    let fields = vec![
        DoField {
            name: "ID".to_string(),
            stype: 4,
            type_: "int".to_string(),
            ..Default::default()
        },
        DoField {
            name: "Name".to_string(),
            stype: 2,
            type_: "int64".to_string(),
            ..Default::default()
        },
    ];
    let doi = Do {
        name: "DoTest".to_string(),
        fields,
        delete_at: true,
    };
    let res = doi.execute();
    println!("{:?}", res);
}
