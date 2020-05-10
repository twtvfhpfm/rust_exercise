
pub fn box_test()
{
    let mut list = ItemList::new();
    list.preappend(CusObj{name: String::from("a")});
    list.preappend(CusObj{name: String::from("b")});
    list.preappend(CusObj{name: String::from("c")});

    println!("total item in list: {}", list.count());

    while let Some(value) = list.take_head() {
        println!("take_head: {:?}", value);
    }

    println!("total item in list: {}", list.count());
}

struct Item<T> {
    value: T,
    next: Option<Box<Item<T>>>,
}

impl<T> Item<T> {
    pub fn set_next(&mut self, item: Box<Item<T>>){
        self.next = Some(item);
    }
}

struct ItemList<T> {
    head: Option<Box<Item<T>>>
}

impl<T> ItemList<T> {
    pub fn new() -> ItemList<T> 
    {
        ItemList {
            head: None
        }
    }

    pub fn preappend(&mut self, value: T)
    {
        let mut item = Item{value:value, next: None};
        if let Some(x) = self.head.take() {
            item.set_next(x);
        }
        //item.unwrap().unwrap().next = old_head;
        self.head = Some(Box::new(item));
    }

    pub fn count(&self) -> i32
    {
        let mut total = 0;
        let mut cursor = self.head.as_ref();

        while let Some(_) = cursor {
            total += 1;
            //cursor = cursor.unwrap().next;
            //cursor = Some(&x.next);
            cursor = cursor.unwrap().next.as_ref();
        }

        total
    }

    pub fn take_head(&mut self) -> Option<T>
    {
        if let Some(item) = self.head.take() {
            self.head = item.next;
            Some(item.value)
        }
        else {
            None
        }
    }
}

#[derive(Debug)]
struct CusObj {
    name: String
}

impl Drop for CusObj {
    fn drop(&mut self){
        println!("{:?} dropped", self);
    }

}