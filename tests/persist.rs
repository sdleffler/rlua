use rlua::Table;

#[test]
fn test_persist_simple_table() {
    rlua::Lua::new().context(|lua| {
        let globals = lua.globals();

        lua.load(
            r#"
                table = {
                    5,
                    a = {
                        "b", "c", "d",
                        ["k"] = { 0, 1, 2, 3 },
                    },
                    [{"wtf"}] = "six",
                }
            "#,
        )
        .exec()
        .unwrap();

        let table = globals.get::<_, Table>("table").unwrap();
        let empty = lua.create_table().unwrap();

        let mut buf = Vec::new();
        lua.dump_value(&mut buf, empty.clone(), table).unwrap();
        let undumped: Table = lua
            .undump_value(&mut buf.as_slice(), empty.clone())
            .unwrap();

        assert_eq!(undumped.get::<_, i32>(1).unwrap(), 5);
        let a = undumped.get::<_, Table>("a").unwrap();
        assert_eq!(a.get::<_, String>(1).unwrap(), "b");
        assert_eq!(a.get::<_, String>(2).unwrap(), "c");
        assert_eq!(a.get::<_, String>(3).unwrap(), "d");
        assert_eq!(&a.get::<_, Vec<i32>>("k").unwrap(), &[0, 1, 2, 3]);
    });
}
