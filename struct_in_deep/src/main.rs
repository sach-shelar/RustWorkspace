
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {

    let user1: User = User {
        email: String::from("sachrohile@gmail.com"),
        username: String::from("sachrohile"),
        sign_in_count: 4,
        active: true,
    };



    let email = &user1.email;
    println!("User1 : {:?}", user1);

    // Build User Instance
    let user2 = build_user(String::from("gloabnt@gmail.com"), String::from("globai"));
    println!("User2 : {:?}",user2);

    //Build instance from another instance
    let user3 = User {
        email: String::from("india@globant.com"),
        ..user2
    };

    println!("User3 : {:?}",user3);

}


fn build_user(email:String,username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
