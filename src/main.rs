#[derive(Clone)]
struct Student {
    name: String,
    age: u32,
    department: String,
}

fn main() {
    println!("Hello, world!");
    let i8_var: i8 = ---12;
    println!("{}", i8_var);
    let u8_var: u8 = 234;
    println!("{}", u8_var);

    let decimal = 6_35;
    println!("{}", decimal);

    let octal = 0o343;
    println!("octal: {}", octal);

    let f_var2: f64 = 2.454;

    println!("{}", f_var2);

    let bool_one = true;
    println!("{}", bool_one);

    let mut tuple1: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}", tuple1.2);

    tuple1.0 = 90;

    println!("{}", tuple1.0);

    let a = [1, 2, 3, 4, 5];
    println!("{}", a[4]);
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i64; 6] = [11, 22, 33, 44, 55, 66];
    println!("{}", a[0]);

    let mut a = [3; 5];
    println!("{}", a[2]);
    a[2] = 7;
    println!("{}", a[2]);
    //let a = [3, 3, 3, 3, 3];

    //&str -> std::str
    let mut camp: &str = "OYK";

    println!("{}", camp);
    camp = "OYK CAMP";
    println!("{}", camp);

    let camp: &'static str = "OYK";
    println!("{}", camp);

    let empty_string = String::new();
    println!("{}", empty_string);

    let mut content_string = String::from("OYK");
    println!("{:?}", content_string);
    content_string = String::from("OYK CAMP");
    println!("{:?}", content_string);

    let var_i = 234223;
    println!("{:?}", var_i.to_string());
    content_string = var_i.to_string();
    println!("{:?}", content_string.to_string());
    let char_var = 'B';
    println!("{:?}", char_var);

    let mut var2 = "ÖYK KIŞ KAMPI".replace("ÖYK", "Özgür Yazılım Kampı");
    println!("{:?}", var2);

    let mut var3 = var2.as_str();
    println!("{}", var3);
    var3 = "DENEME";
    println!("{}", var2);
    println!("{}", var3);
    let mut var4 = var3;
    println!("var4: {}", var4);
    var4 = "XX";
    println!("{} {}", var3, var4);
    println!("{var3} {var2}");

    var2.push(' ');
    println!("{}", var2);
    let var5 = var2.trim();
    //var2.push_str("ÖYK");
    println!("{}", var2.len());

    println!("{}", var2);
    println!("{}", var5);
    println!("{:?}", var5.split(' '));
    println!("{:?}", var5.chars());

    let var6 = format!("test");
    println!("{}", var6);
    let var7 = format!("hello {}", "world!");
    println!("{}", var7);

    let var8 = format!("x = {1}, y = {val}, z={0}", 10, val = 30);
    println!("{}", var8);

    let (x, y) = (1, 2.0);
    println!("{} - {}", x, y);

    let num = 5;
    if num > 0 {
        println!("number is positive");
    }

    let num = 13;
    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    let num = 0;
    if num > 0 {
        println!("{} is positive", num);
    } else if num < 0 {
        println!("{} is negative", num);
    } else {
        println!("{} is neither positive nor negative", num);
    }
    // Elimizde bir sayı var ve bu sayı octal olarak veriliyor
    //bu sayının 5 e bölümünden kalanı 1 ise
    //"bu sayının kanalı 1" yazsın
    // kalan kısmı diğerleriyse ekranı kalan yazdırılsın
    // tam bölünüyorsa ise "tam bölündü" yazsın

    let country_code = "TRX";
    let code = match country_code {
        "TR" => {
            println!("Found match for TR");
            "Türkiye"
        }
        "US" => "Amerika",
        _ => "Bilinmeyen",
    };
    println!("State name is {}", code);

    let mut yas = 10;
    if yas > 18 {
        println!("18den büyük");
    } else {
        println!("18den küçük");
    };

    yas = 23;
    let yas_str = match yas {
        0..=17 => "18den küçük",
        _ => "18den büyük",
    };

    println!("{}", yas_str);

    let code = 26;

    let code_str = match code {
        6 => "Başkent",
        34 => "İstanbul",
        _ => "Diğer Şehirler",
    };
    println!("{}", code_str);
    let mut toplam = 0;
    for x in 1..11 {
        // 11 is not inclusive
        if x % 2 == 0 {
            toplam += (x * 3);
        }
        if x % 2 == 1 {
            toplam = toplam + (x * 7);
        }
    }
    println!("toplam: {}", toplam);

    let mut x = 0;
    let mut y = 1;
    while x < 10 && y < 4 {
        x += 1;
        println!("inside loop x value is {}", x);
        if x % 2 == 0 {
            y += 1;
        }
    }
    println!("outside loop x value is {}", x);

    let mut x = 0;
    loop {
        x += 1;
        println!("x={}", x);

        if x == 15 {
            break;
        }
    }
    //Bizim 10 elemanlı bir döngümüz olsun.
    //Bu döngü ile bizim o anki index değerimiz çiftse 3 ile çarpsin
    //tek ise 7 ile çarpsın ve bu işlemler sonucundaki sayıların
    // toplamını ekrana yazdıran bir for döngüsü yazalım

    //Verdiğimiz bir sayının içerisindeki çift sayıların
    //kaç tane olduğunu bulan ve tek sayıları da ekrana yazdıran
    //bir kod yazınız.

    let sayi = 10;
    let mut toplam = 0;
    for x in 1..=sayi {
        if x % 2 == 0 {
            toplam += 1;
            continue;
        }
        println!("tek sayı bulundu: {}", x);
    }
    println!("toplam çift sayı: {}", toplam);

    fn_hello();
    let pi_value = get_pi();
    println!("{}", pi_value.0);

    //let no: i32 = 5;
    //mutate_no_to_zero(no);
    // println!("The value of no is:{}", no);

    let mut no: i32 = 65;
    mutate_no_to_zero_with_ref(&mut no);
    println!("The value of no is:{}", no);

    let mut name: String = String::from("ÖYK Rust 101");
    name = display(name);
    println!(" before value: {}", name);

    struct Kamp {
        field1: i32,
        field2: String,
        field3: f64,
    }

    let mut kamp = Kamp {
        field1: 32,
        field2: String::from("ÖYK Rust 101"),
        field3: 3.5,
    };
    println!("1: {}", kamp.field1);

    kamp.field1 = 42;
    println!("1.1 : {}", kamp.field1);

    let student1 = Student {
        name: String::from("Öğrenci 1"),
        age: 24,
        department: String::from("Computer Engineering"),
    };

    let student2 = Student {
        name: String::from("Öğrenci 2"),
        age: 19,
        department: String::from("Chemical Engineering"),
    };

    let student3 = Student {
        name: String::from("Öğrenci 3"),
        age: 32,
        department: String::from("Electric - Electronic Engineering"),
    };

    let array = [student1.clone(), student2, student3];
    let array2 = array.clone();
    for student in array {
        println!("{}", format_with_struct(student));
    }

    let student_var = get_student("ÖYK Kamp".to_string(), "Rust".to_string(), 1986);
    let student_info = format_with_struct(student_var);
    println!("{}", student_info);

    for year in 1985..=1995 {
        let student = get_student("Öğrenci -".to_string(), "Rust".to_string(), year);
        let student_info = format_with_struct(student);
        println!("{}", student_info);
    }
    //array2 içerisindeki öğrencilerin yaşlarının ortalamasını
    //döndüren fonksiyon yazılıp çağrılması.

    //println!("{}",format_with_struct(array2[0].to_owned()))

    #[derive(Debug)]
    enum GenderCategory {
        Male,
        Female,
    }
    let male = GenderCategory::Male;
    let female = GenderCategory::Female;

    println!("{:?}", male);
    println!("{:?}", female);

    let is_even_var = is_even(5);
    println!("{:?}", is_even_var);

    print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);

    match is_even(6) {
        Some(data) => {
            if data == true {
                println!("Even no");
            }
        }
        None => {
            println!("not even");
        }
    }

    let p1: PersonCategory = PersonCategory::Name(String::from("ÖYK"));
    let p2 = PersonCategory::Count(18);

    println!("{:?}", p1);
    println!("{:?}", p2);
let ders1= Camp::Ders("Rust101".to_string(), 14);
let ders2 = Camp::Ders("Rust ile Blockchain".to_string(), 16);
let hoca1 = Camp::Hoca("Veli UYSAL".to_string());
let hoca2 = Camp::Hoca("Aydın YAKAR".to_string());

print_camp_enum(ders1);
print_camp_enum(ders2);
print_camp_enum(hoca1);
print_camp_enum(hoca2);

}

fn print_camp_enum(camp: Camp){
    match camp {
        Camp::Ders(name,count ) => {
            println!("Ders adı: {}, Mevcut: {}",name, count);
        },
        Camp::Hoca(name) => {
            println!(" Hoca: {}",name);
        }
    }
}

#[derive(Debug)]
enum PersonCategory {
    Name(String),
    Count(i32),
}

enum Camp {
    Ders(String, u32),
    Hoca(String)
}

enum CarType {
    Hatch,
    Sedan,
    SUV,
}

fn print_size(car: CarType) {
    match car {
        CarType::Hatch => {
            println!("Small sized car");
        }
        CarType::Sedan => {
            println!("medium sized car");
        }
        CarType::SUV => {
            println!("Large sized Sports Utility car");
        }
    }
}

fn is_even(no: i32) -> Option<bool> {
    if no % 2 == 0 {
        Some(false)
    } else {
        None
    }
}

pub fn fn_hello() {
    println!("hello from function fn_hello ");
}

fn get_pi() -> (f64, i32) {
    let pivar = 22.0 / 7.0;
    (pivar, 2)
}

fn mutate_no_to_zero(mut param_no: i32) {
    param_no = param_no * 0;
    println!("param_no value is :{}", param_no);
}

fn mutate_no_to_zero_with_ref(param_no: &mut i32) {
    *param_no = 0; //de reference
}

fn display(mut param_name: String) -> String {
    param_name = String::from("Deneme");
    println!("param_name value is :{}", param_name);
    param_name
}

fn format_with_struct(student: Student) -> String {
    format!(
        "Adı: {}, Departmanı: {}, Yaşı: {}",
        student.name, student.department, student.age
    )
}

//fn array_fn(array: [Student; 3]) -> i32 {}

fn get_student(name: String, department: String, birth_year: u32) -> Student {
    let student = Student {
        name: name,
        department: department,
        age: 2024 - birth_year,
    };
    student
}
