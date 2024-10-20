fn main() {
    use std::clone;

fn main() {


    let mut database: Vec<User> = Vec::new();


    register("yigit".to_string(), "yahyayigityildiz1".to_string()
    , "yigit123".to_string(),&mut database);

    login("yahyayigityildiz1".to_string(), "yigit123".to_string(), 
    &mut database);
    
}



#[derive(Clone,Debug)]
struct User {
    name:String,
    email:String,
    password:String
}


fn login(email:String,password:String,database : &mut Vec<User>) {

    for data in database  {

        if email == data.email && password== data.password {

            println!("Tebrikler giriş yaptınız");
            
        }
        else {
            println!("Tekrar deneyin")
        }
        
    }
}

fn register(name:String,email:String,
    password:String,database : &mut Vec<User>) {


    if !email.contains("@") {

        println!("yanlış email formatı")
        
    }
    else {
        let user = User{
            name:name,
            email:email,
            password:password
        };

        let user1= user.clone();

        database.push(user1);

        println!("Kullanıcı başarılı kayıt oldu {:?} " , database )
    }
    
}
}
