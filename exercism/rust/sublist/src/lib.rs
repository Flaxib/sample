#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

use std::fmt::Debug;

pub fn sublist<T: PartialEq + Debug>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        if _first_list == _second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if _first_list.len() < _second_list.len() {
        let nb_test = _second_list.len() - _first_list.len() + 1;
        let mut res = false;
        for i in 0..nb_test {
            let sublist = &_second_list[i..(i + _first_list.len())];
            if _first_list == sublist {
                res = true;
                break;
            }
        }
        if res {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else {
        let res = sublist(_second_list, _first_list);
        match res {
            Comparison::Sublist => Comparison::Superlist,
            _ => Comparison::Unequal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equallity() {
        assert_eq!(sublist(&[1, 2, 3], &[1, 2, 3]), Comparison::Equal);
    }
    #[test]
    fn sublist0() {
        assert_eq!(sublist(&[1, 2, 3], &[1, 2, 4]), Comparison::Unequal);
    }
    #[test]
    fn sublist1() {
        assert_eq!(sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]), Comparison::Sublist);
    }
    #[test]
    fn sublist2() {
        assert_eq!(sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]), Comparison::Sublist);
    }
    #[test]
    fn sublist3() {
        assert_eq!(sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]), Comparison::Superlist);
    }
    #[test]
    fn sublist4() {
        assert_eq!(sublist(&[1, 2, 3, 4, 5], &[3, 4, 5]), Comparison::Superlist);
    }
}
