pub struct Reader<A,B> {
    run: Fn(A) -> B,
    //run: Box<Fn(&R) -> A + 'reader>,
   // _marker: PhantomData<R>,
    //content_type: PhantomData<A>,
}

impl<A,B> Reader<A,B> {
    pub fn pure<F>(f: F) -> Reader<A,B>
        where
            F: Fn(A) -> B,
    {
        Reader {
            run: f,
            //state_type: PhantomData,
            //content_type: PhantomData,
        }
    }

    pub fn run(&self, a:A) -> B {
        (self.run)(a)
    }

    // pub fn map<B: 'reader, G>(self, f: G) -> Reader<'reader, R, B>
    //     where
    //         G: Fn(A) -> B + 'reader,
    // {
    //     let h = move |s: &R| {
    //         let a = (self.run)(s);
    //         f(a)
    //     };
    //     Reader::new(h)
    // }
    //
    // pub fn flat_map<B: 'reader, G>(self, f: G) -> Reader<'reader, R, B>
    //     where
    //         G: Fn(A) -> Reader<'reader, R, B> + 'reader,
    // {
    //     let h = move |s: &R| {
    //         let a = (self.run)(s);
    //         (f(a).run)(s)
    //     };
    //     Reader::new(h)
    // }
}


#[cfg(test)]
mod tests {

    use super::Reader;

    #[derive(Debug)]
    struct Connection {
        state: i32,
    }
    struct User {id:i32}

    fn get_user(Connection c,id: i32) -> User {
        User{id:c.state}
    }

    //fn get_user(id: i32) -> Reader<Connection, User> {
        Reader::new(move |c: &Connection| User{})
    }
    // fn get_other(id: i32) -> Reader<'static, Connection, i32> {
    //     Reader::new(move |c: &Connection| id + c.x)
    // }

    #[test]
    fn it_works() {

        let conn = Connection { state:0 };

        let r = Reader::pure(move |c| get_user(c,10));
        let u = r.run(conn);
        //let r = get_user(10).map(|a| a + 20);
        //let r = get_user(10).flat_map(|a| get_other(a + 1000));

        //let u = (r.run)(&conn);


        println!("##### Result: {:?} ", u);
        assert!(true);
    }
}