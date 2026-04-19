use dotenvy::dotenv;
use std::env;

const MONTHS_IN_YEAR: i32 = 12;
const PERCENTAGE_MAX: f64 = 100.0;
const RATE_MAX: f64 = 1.0;

fn main() {
    dotenv().ok();

    let principal: f64 = env::var("PRINCIPAL").expect("PRINCIPAL variable not found").parse().expect("A floating point number");
    let annual_interest_rate_percentage: f64 = env::var("INTEREST_RATE_PERCENTAGE").expect("INTEREST_RATE_PERCENTAGE variable not found").parse().expect("A floating point number");
    let annual_interest_rate: f64 = annual_interest_rate_percentage / PERCENTAGE_MAX;

    let years_of_loan: i32 = env::var("NUMBER_OF_YEARS").expect("NUMBER_OF_YEARS variable not found").parse().expect("An integer was expected"); 
    let months_of_loan: i32 = years_of_loan * MONTHS_IN_YEAR;

    let monthly_rate = annual_interest_rate / 12.0;
    let monthly_payment = principal * (monthly_rate * (RATE_MAX + monthly_rate).powi(months_of_loan)) / ((RATE_MAX + monthly_rate).powi(months_of_loan) - RATE_MAX);

    let extra_yearly_payment_amount: f64 = env::var("ADDITIONAL_YEARLY_PAYMENTS").expect("ADDITIONAL_YEARLY_PAYMENTS variable not found").parse().expect("A floating point number");

    println!("Standard Monthly Payment: ${:.2}", monthly_payment);

    let (total_interest_standard, _) = simulate_mortgage(principal, months_of_loan, monthly_rate, monthly_payment, 0.0);
    let (total_interest_extra, months_saved) = simulate_mortgage(principal, months_of_loan, monthly_rate, monthly_payment, extra_yearly_payment_amount * monthly_payment);

    println!("Total Interest (Standard): ${:.2}", total_interest_standard);
    println!("Total Interest (Extra Payment/Year): ${:.2}", total_interest_extra);
    println!("Total Savings: ${:.2}", total_interest_standard - total_interest_extra);
    println!("Time Saved: {} months ({:.1} years)", months_saved, months_saved as f64 / 12.0);
}

fn simulate_mortgage(mut balance: f64, total_months: i32, monthly_rate: f64, monthly_payment: f64, extra_annual_payment: f64) -> (f64, i32) {
    let mut total_interest = 0.0;
    let mut month = 0;

    while balance > 0.01 && month < 600 { // 600 is a safety break
        month += 1;
        let interest_payment = balance * monthly_rate;
        total_interest += interest_payment;
        
        let principal_payment = (monthly_payment - interest_payment).min(balance);
        balance -= principal_payment;

        // Apply extra payment once a year (e.g., every 12th month)
        if extra_annual_payment > 0.0 && month % 12 == 0 && balance > 0.0 {
            let extra = extra_annual_payment.min(balance);
            balance -= extra;
        }
    }

    (total_interest, total_months - month)
}
