extern crate mps;

#[test]
fn wiki_mps() {
    let file = std::fs::File::open("tests/wiki.mps").unwrap();
    let result = mps::read(file);
    let prob = result.expect("should be ok");
    assert_eq!(prob.name, "TESTPROB");
    {
        let mut iter = prob.rows_by_index.iter();
        let row = iter.next().expect("no COST row");
        assert_eq!(row.name, "COST");
        assert_eq!(row.typ, mps::RowType::Objective);
        let row = iter.next().expect("no LIM1 row");
        assert_eq!(row.name, "LIM1");
        assert_eq!(row.typ, mps::RowType::Le);
        let row = iter.next().expect("no LIM2 row");
        assert_eq!(row.name, "LIM2");
        assert_eq!(row.typ, mps::RowType::Ge);
        let row = iter.next().expect("no MYEQN row");
        assert_eq!(row.name, "MYEQN");
        assert_eq!(row.typ, mps::RowType::Eq);
        assert_eq!(iter.next(), None);
    }
    {
        let mut cols = prob.columns_by_id.iter();
        let (id, col) = cols.next().expect("no XONE col");
        assert_eq!(id, "XONE");
        {
            let mut data = col.data.iter();
            assert_eq!(data.next(), Some((&0, &1.0)));
            assert_eq!(data.next(), Some((&1, &1.0)));
            assert_eq!(data.next(), Some((&2, &1.0)));
            assert_eq!(data.next(), None);
        }
        let (id, col) = cols.next().expect("no YTWO col");
        assert_eq!(id, "YTWO");
        {
            let mut data = col.data.iter();
            assert_eq!(data.next(), Some((&0, &4.0)));
            assert_eq!(data.next(), Some((&1, &1.0)));
            assert_eq!(data.next(), Some((&3, &-1.0)));
            assert_eq!(data.next(), None);
        }
        let (id, col) = cols.next().expect("no ZTHREE col");
        assert_eq!(id, "ZTHREE");
        {
            let mut data = col.data.iter();
            assert_eq!(data.next(), Some((&0, &9.0)));
            assert_eq!(data.next(), Some((&2, &1.0)));
            assert_eq!(data.next(), Some((&3, &1.0)));
            assert_eq!(data.next(), None);
        }
        assert_eq!(cols.next(), None);
    }
    {
        let mut rhss = prob.rhs_by_id.iter();
        let (id, rhs) = rhss.next().expect("no RHS1 rhs");
        assert_eq!(id, "RHS1");
        {
            let mut data = rhs.data.iter();
            assert_eq!(data.next(), Some((&1, &5.0)));
            assert_eq!(data.next(), Some((&2, &10.0)));
            assert_eq!(data.next(), Some((&3, &7.0)));
            assert_eq!(data.next(), None);
        }
        assert_eq!(rhss.next(), None);
    }
}