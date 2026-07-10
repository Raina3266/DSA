// type List = Option<Box<ListNode>>
// struct ListNode {
//     head: u8,
//     tail: List,
// }

// impl ListNode {

// }

enum List {
    Nil,
    Cons {head: u8, tail: Option<Box<List>>},
}


pub fn add_two_numbers(l1: List, l2: List) -> List {
    let mut list = List::Nil;
    match (l1, l2) {
        (List::Nil, List::Nil) => return list,
        (List::Nil, List::Cons {head, tail}) => return list,
        (List::Cons {head, tail}, List::Nil) => return list,
        (List::Cons {head1, tail1}, List::Cons {head2, tail2}) => {
            
        },
    }

    
    let head_l1 = match l1 {
        List::Nil => 0,
        List::Cons {head, tail} => head,
    };
    let head_l2 = match l2 {
        List::Nil => 0,
        List::Cons {head, tail} => head,
    };

    let mut add_one = false;

    let sum = if add_one {
        l1 + l2 + 1;
    } else {
        l1 + l2
    };

    

    if sum >= 10 {
        add_one = true
    }
}
