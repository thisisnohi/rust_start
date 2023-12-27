pub mod hosting;

mod serving {
    pub(crate) fn take_order() {
        println!("serving take_order")
    }

    fn serve_order() {}

    fn take_payment() {}
}
