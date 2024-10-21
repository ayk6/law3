#![no_std]

use soroban_sdk::{contractimpl, Env, Address, Symbol};

pub struct AttorneyClientContract;

// Contract structure to represent the agreement between the attorney and the client
#[derive(Clone)]
pub struct Contract {
    attorney_name: Symbol,       // The name of the attorney
    client_name: Symbol,         // The name of the client
    institution: Symbol,         // The institution associated with the contract
    case_number: Symbol,         // Unique identifier for the case
    contract_fee: u64,           // Fee for the contract
    payment_method: Symbol,      // Method of payment (e.g., bank transfer, cash)
    penalty_clause: Symbol,      // Terms for penalties in case of contract breach
    dispute_resolution: Symbol,  // Authority for dispute resolution
}

// Appointment structure to represent a client's consultation appointment
#[derive(Clone)]
pub struct Appointment {
    client_name: Symbol,               // The name of the client
    consultation_topic: Symbol,        // Topic of the consultation
    start_date: Symbol,                // Date and time when the consultation starts
    total_duration: u64,               // Total duration of the consultation in minutes
    consultation_fee: u64,             // Fee for the consultation
    payment_method: Symbol,             // Method of payment for the consultation
    consultation_type: Symbol,          // Type of consultation (online or face-to-face)
}


#[contractimpl]
impl AttorneyClientContract {
    // Function to create a contract between an attorney and a client
    pub fn create_contract(
        env: Env,
        attorney_name: Symbol,
        client_name: Symbol,
        institution: Symbol,
        case_number: Symbol,
        contract_fee: u64,
        payment_method: Symbol,
        penalty_clause: Symbol,
        dispute_resolution: Symbol,
    ) {
        // Create a new contract instance with the provided details
        let contract = Contract {
            attorney_name,
            client_name,
            institution,
            case_number,
            contract_fee,
            payment_method,
            penalty_clause,
            dispute_resolution,
        };
        // Store the contract in the blockchain with the case number as the key
        env.storage().set(case_number, contract);
    }

    // Function to retrieve a contract based on the case number
    pub fn get_contract(env: Env, case_number: Symbol) -> Option<Contract> {
        // Retrieve the contract from storage using the case number
        env.storage().get(case_number)
    }

    // Function to create an appointment for a client consultation
    pub fn create_appointment(
        env: Env,
        client_name: Symbol,
        consultation_topic: Symbol,
        start_date: Symbol,
        total_duration: u64,
        payment_method: Symbol,
        consultation_type: Symbol, // 'online' or 'in-person'
    ) -> Result<u64, &'static str> {
        // Check if an appointment already exists at the specified time
        if let Some(existing_appointment) = env.storage().get::<Appointment>(client_name) {
            return Err("Appointment time is already booked."); // Return error if the time is booked
        }

        // Calculate the total fee based on duration
        let mut total_fee = 0;

        if total_duration > 0 {
            total_fee += 3500; // First hour fee
            if total_duration > 60 {
                let additional_hours = (total_duration - 60) / 60; // Calculate additional hours
                total_fee += additional_hours * 1500; // Additional fee for each hour
            }
        }

        // Create a new appointment instance with the provided details
        let appointment = Appointment {
            client_name,
            consultation_topic,
            start_date,
            total_duration,
            consultation_fee: total_fee,
            payment_method,
            consultation_type,
        };
        // Store the appointment in the blockchain with the client name as the key
        env.storage().set(client_name, appointment);
        
        Ok(total_fee) // Return the total fee as a success response
    }


    // Function to retrieve an appointment based on the client name
    pub fn get_appointment(env: Env, client_name: Symbol) -> Option<Appointment> {
        // Retrieve the appointment from storage using the client name
        env.storage().get(client_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, testutils::*, Bytes};

    // Test for creating an appointment
    #[test]
    fn test_create_appointment() {
        let env = Env::default();
        let contract = AttorneyClientContract::new(&env);

        // Define appointment parameters
        let client_name = Bytes::from("John Doe");
        let consultation_topic = Bytes::from("Legal Advice");
        let start_date = Bytes::from("2024-01-01T10:00:00Z");
        let total_duration = 120; // 2 hours
        let payment_method = Bytes::from("Credit Card");
        let consultation_type = Bytes::from("online");

        // Create an appointment
        let fee = contract.create_appointment(
            &env,
            client_name.clone(),
            consultation_topic.clone(),
            start_date.clone(),
            total_duration,
            payment_method.clone(),
            consultation_type.clone(),
        ).unwrap();

        // Assert that the fee is calculated correctly
        assert_eq!(fee, 3500 + 1500); // First hour + additional hour
    }

    // Test for appointment time conflict
    #[test]
    fn test_appointment_time_conflict() {
        let env = Env::default();
        let contract = AttorneyClientContract::new(&env);

        // Create the first appointment
        let client_name = Bytes::from("My Client");
        let consultation_topic = Bytes::from("Legal Advice");
        let start_date = Bytes::from("2024-01-01T10:00:00Z");
        let total_duration = 120; // 2 hours
        let payment_method = Bytes::from("Credit Card");
        let consultation_type = Bytes::from("online");

        // Create the first appointment
        let _ = contract.create_appointment(
            &env,
            client_name.clone(),
            consultation_topic.clone(),
            start_date.clone(),
            total_duration,
            payment_method.clone(),
            consultation_type.clone(),
        ).unwrap();

        // Attempt to create a conflicting appointment
        let result = contract.create_appointment(
            &env,
            Bytes::from("Jane Doe"), // Different client
            consultation_topic,
            start_date,
            total_duration,
            payment_method,
            consultation_type,
        );

        // Assert that an error is returned
        assert_eq!(result.unwrap_err(), "Appointment time is already booked.");
    }
}

