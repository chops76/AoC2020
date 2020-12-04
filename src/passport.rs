#[derive(Debug)]
#[derive(Default)]
pub struct Passport {
    pub byr: Option<String>,
    pub iyr: Option<String>,
	pub eyr: Option<String>,
	pub hgt: Option<String>,
    pub hcl: Option<String>,
	pub ecl: Option<String>,
	pub pid: Option<String>,
    pub cid: Option<String>
}

impl Passport
{
    pub fn new(items: Vec<&str>) -> Self {
        let mut byr: Option<String> = None;
        let mut iyr: Option<String> = None;
        let mut eyr: Option<String> = None;
        let mut hgt: Option<String> = None;
        let mut hcl: Option<String> = None;
        let mut ecl: Option<String> = None;
        let mut pid: Option<String> = None;
        let mut cid: Option<String> = None;
        for field in items {

            let vals: Vec<&str> = field.split(':').collect();
            match vals[0] {
                "byr" => byr = Some(vals[1].to_string()),
                "iyr" => iyr = Some(vals[1].to_string()),
                "eyr" => eyr = Some(vals[1].to_string()),
                "hgt" => hgt = Some(vals[1].to_string()),
                "hcl" => hcl = Some(vals[1].to_string()),
                "ecl" => ecl = Some(vals[1].to_string()),
                "pid" => pid = Some(vals[1].to_string()),
                "cid" => cid = Some(vals[1].to_string()),
                _ => println!("Something terrible happened: {}", vals[0])	
            }
        }
        Self {
            byr: byr,
            iyr: iyr,
            eyr: eyr,
            hgt: hgt,
            hcl: hcl,
            ecl: ecl,
            pid: pid,
            cid: cid
        }
    }
    
    pub fn fields_populated(&self) -> bool {
        self.byr != None && self.iyr != None && self.eyr != None && 
            self.hgt != None && self.hcl != None && self.ecl != None &&
		    self.pid != None 
    }

    pub fn validate_byr(&self) -> bool {
        self.byr.as_ref().unwrap().len() == 4 && 
            (1920..=2002).contains(&self.byr.as_ref().unwrap().parse::<u32>().unwrap())
    }
    
    pub fn validate_iyr(&self) -> bool {
        self.iyr.as_ref().unwrap().len() == 4 && 
            (2010..=2020).contains(&self.iyr.as_ref().unwrap().parse::<u32>().unwrap())
    }
    
    pub fn validate_eyr(&self) -> bool {
        self.eyr.as_ref().unwrap().len() == 4 && 
            (2020..=2030).contains(&self.eyr.as_ref().unwrap().parse::<u32>().unwrap())
    }
    
    pub fn validate_hgt(&self) -> bool {
        match self.hgt.as_ref().unwrap().split_at(self.hgt.as_ref().unwrap().len() - 2) {
            (h, "cm") => (150..=193).contains(&h.parse::<u32>().unwrap()),
            (h, "in") => (59..=76).contains(&h.parse::<u32>().unwrap()),
            _ => false
        }
    }
    
    pub fn validate_hcl(&self) -> bool {
        self.hcl.as_ref().unwrap().chars().next() == Some('#') &&
            self.hcl.as_ref().unwrap().chars().skip(1).all(|c| !c.is_uppercase() && c.is_ascii_hexdigit() )
    }
    
    pub fn validate_ecl(&self) -> bool {
        let tmp = self.ecl.as_ref().unwrap();
        tmp == "amb" || tmp == "blu" || tmp == "brn" || tmp == "gry" ||
            tmp == "grn" || tmp == "hzl" || tmp == "oth"
    }
    
    pub fn validate_pid(&self) -> bool {
        self.pid.as_ref().unwrap().len() == 9 && self.pid.as_ref().unwrap().chars().all(|c| c.is_numeric())
    }

    pub fn validate(&self) -> bool {
        self.fields_populated() && 
            self.validate_byr() &&
            self.validate_iyr() && 
            self.validate_eyr() &&
            self.validate_hgt() &&
            self.validate_hcl() &&
            self.validate_ecl() &&
            self.validate_pid()
    }
}
