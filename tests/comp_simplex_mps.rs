use mps::RowDef;

extern crate mps;

#[test]
fn wiki_mps() {
    let file = std::fs::File::open("tests/comp_simplex.mps").unwrap();
    let result = mps::read(file);
    let prob = result.expect("should be ok");
    assert_eq!(prob.name, "Sample Problem");
    assert_eq!(
        prob.rows_by_index,
        [
            RowDef {
                name: "OBJ".to_string(),
                index: 0,
                typ: mps::RowType::Objective
            },
            RowDef {
                name: "Res-1".to_string(),
                index: 1,
                typ: mps::RowType::Le
            },
            RowDef {
                name: "Res-2".to_string(),
                index: 2,
                typ: mps::RowType::Le
            },
            RowDef {
                name: "Res-3".to_string(),
                index: 3,
                typ: mps::RowType::Le
            },
            RowDef {
                name: "Balance".to_string(),
                index: 4,
                typ: mps::RowType::Eq
            }
        ]
        .map(std::rc::Rc::new)
    );
    {
        let mut cols = prob.columns_by_id.iter();
        let (id, col) = cols.next().unwrap();
        assert_eq!(id, "Vol--1");
        assert_eq!(col.data, [(0, 4.5), (1, 1.0), (4, 2.5)].into());
        let (id, col) = cols.next().unwrap();
        assert_eq!(id, "Vol--2");
        assert_eq!(col.data, [(0, 2.5), (2, 1.5), (4, 2.0)].into());
        let (id, col) = cols.next().unwrap();
        assert_eq!(id, "Vol--3");
        assert_eq!(col.data, [(0, 4.0), (1, 1.0), (2, 0.5), (3, 3.0)].into());
        let (id, col) = cols.next().unwrap();
        assert_eq!(id, "Vol--4");
        assert_eq!(col.data, [(0, 4.0), (1, 1.5), (2, 0.5), (3, 2.0)].into());
        assert_eq!(cols.next(), None);
    }
    {
        let mut rhss = prob.rhs_by_id.iter();
        let (id, rhs) = rhss.next().unwrap();
        assert_eq!(id, "RHS-1");
        assert_eq!(rhs.data, [(1, 40.0), (2, 30.0), (4, 95.0)].into());
        assert_eq!(rhss.next(), None);
    }
}
