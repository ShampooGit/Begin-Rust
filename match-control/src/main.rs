//
// Match statements
//
/*
the big moment ive been waiting for kraaaa,
im really excited about this section
after this chapter i will prob try to make some functional application
*/

// 

fn main() {
    enum eu_cent {
        5_cent,
        10_cent,
        20_cent,
        50_cent,
    }
    fn value(eu_cents: eu_cent) -> u32 {
        match eu_cents {
            eu_cent::5_cent => 5,
            eu_cent::10_cent => 10,
            eu_cent::20_cent => 20,
            eu_cent::50_cent => 50,
        }
    }
}
