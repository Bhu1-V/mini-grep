use std::fs;
use std::error::Error;

pub struct Config {
    expression : String,
    file_name : String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not Enough Arguments. ");
        }
        
        Ok(
            Config {
                expression : args[1].clone(),
                file_name : args[2].clone(),
            }
        )
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(&self.file_name)?;

        let res = Config::search(self,&contents);
        
        if res.len() != 0{
            println!("\nFound {} Results\n",res.len());
            for line in Config::search(&self,&contents){
                if !line.is_empty(){
                    println!("{}",line);
                }
            }
        }else{
            println!("Can't Find {} in {}",self.expression,self.file_name);
        }

        Ok(())
    }

    fn search<'a>(&self,contents:&'a str) -> Vec<&'a str>{
        let mut res:Vec<&str> = Vec::new();

        for mut line in contents.lines(){
            if line.contains(&self.expression){
                line = line.trim();
                res.push(line);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let con = Config{
            expression : "duct".to_string(),
            file_name : "nun".to_string(),
        };

        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."],con.search(contents));

    }

    #[test]
    fn case_sensitive(){
        
    }
}