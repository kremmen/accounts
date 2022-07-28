use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::serializer::*;

use crate::{account::{Schedule, Transaction}, books::BooksError};

/// 

#[derive(Serialize, Deserialize)]
pub struct Scheduler {
    schedules: Vec<Schedule>,
    #[serde(serialize_with = "serialize_option_naivedate")]
    #[serde(deserialize_with = "deserialize_option_naivedate")]    
    end_date: Option<NaiveDate>
}

impl Scheduler {
    
    pub fn build_empty() -> Scheduler {
        Scheduler{schedules: Vec::new(), end_date: None}        
    }

    pub fn add_schedule(&mut self, schedule: Schedule) {    
        self.schedules.push(schedule);
    }
    
    pub fn update_schedule(&mut self, schedule: Schedule) -> Result<(), BooksError> {
    
        if let Some(index) = self.schedules.iter().position(|s| s.id == schedule.id) {
            let _old = std::mem::replace(&mut self.schedules[index], schedule);
            Ok(())          
        } else {
            Err(BooksError { error: "Schedule not found".to_string() })        
        }
        
    }
    
    pub fn schedules(&self) -> &[Schedule] {
        self.schedules.as_slice()
    }

    pub fn end_date(&self) -> Option<NaiveDate> {
        println!("{:?}", self.end_date);
        self.end_date.and_then(|d| Some(d.clone()))        
    }

	pub fn generate(&mut self, end_date: NaiveDate) -> Vec<Transaction> {
		let mut transactions : Vec<Transaction> = Vec::new();
        self.end_date = Some(end_date);

		for schedule in &mut self.schedules {
			let mut next = schedule.schedule_next(end_date);
			while next.is_some() {
				transactions.push(next.unwrap());
				next = schedule.schedule_next(end_date);
			}
		}
        transactions.sort_by(|a, b| a.date.cmp(&b.date));        
        transactions
	}
}



#[cfg(test)]

mod tests {
    use uuid::Uuid;
    use chrono::{NaiveDate};
    use rust_decimal_macros::dec;
    use crate::{account::*, scheduler::Scheduler};

     
    #[test]
	fn test_generate() {
        let mut scheduler  = Scheduler{
            schedules: Vec::new(),
            end_date: None
        };
		let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        scheduler.schedules.push(
            Schedule {
                id: Uuid::new_v4(),
                name: "S_1".to_string(),
                period: ScheduleEnum::Months,
                frequency: 3,
                start_date:   NaiveDate::from_ymd(2022, 3, 11),
                last_date:   None,
                amount:      dec!(100.99),
                description: "st test 1".to_string(),
                dr_account_id: Some(id1),
                cr_account_id: Some(id2)
            });

        scheduler.schedules.push(
            Schedule {
                id: Uuid::new_v4(),
                name: "S_2".to_string(),
                period: ScheduleEnum::Days,
                frequency: 45,
                start_date:   NaiveDate::from_ymd(2022, 3, 11),
                last_date:   None,
                amount:      dec!(20.23),
                description: "st test 2".to_string(),
                dr_account_id: Some(id2),
                cr_account_id: Some(id1)
            });

        let transactions = scheduler.generate(NaiveDate::from_ymd(2023, 3, 11));		
		
		assert_eq!(14, transactions.len());
		assert_eq!("st test 2", transactions[2].description);
		assert_eq!("st test 1", transactions[4].description);
	}

}