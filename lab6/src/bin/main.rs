use lab6::account::*;

fn main() {
    let user1 = new_account("zack", "cne3bfvkw5eerv30jy3990yz0w");
    let user2 = parse_account("sam:bf6ddgabh6jghr89beyg3gn7e8").unwrap();
    let user1_name = &user1.username;
    let user2_name = &user2.username;
    println!("let's welcome our new users: {user1_name} and {user2_name}");
    //let user1_pwd = &user1.password; // error[E0616]: field `password` of struct `Account` is private
}