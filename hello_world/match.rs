fn match_statement()
{
    let country_code = 12000;
    let country = match country_code
        {
            44 => "UK",
            46 => "Sweden",
            7 => "Ruassia",
            _ => "unknown",
            1..=999 => "don't know"//1..999代表1到998；1...999代表1到999
        };

println!("the country with code {} is {}", country_code, country);
}
fn main() {
    match_statement();
}