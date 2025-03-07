use lib_proto::FileMetadata;

use crate::employee;
use crate::file;
use crate::department;
use crate::emergency_information;
use crate::job_information;

impl Into<lib_proto::Department> for department::Model{
    fn into(self) -> lib_proto::Department {
        lib_proto::Department {
            id: self.id.to_string(),
            name: self.name.to_string(),
            description: self.description,
            employee_ids: self.employee_ids.iter().map(|id| id.to_string()).collect(),
        }
    }
}

impl Into<lib_proto::JobInformation> for job_information::Model{
    fn into(self) -> lib_proto::JobInformation {
        lib_proto::JobInformation {
            id: self.id.to_string(),
            title: self.title.to_string(),
            employee_id: self.employee_id.to_string(),
            department_id: self.department_id.to_string(),
            supervisor_id: self.supervisor_id.to_string(),
            work_location: self.work_location.to_string(),
            start_date: self.start_date.to_string(),
            salary: self.salary.to_string(),
            currency: self.currency.to_string(),
        }
    }
}

impl Into<lib_proto::EmployeeEmergencyInformation> for emergency_information::Model{
    fn into(self) -> lib_proto::EmployeeEmergencyInformation {
        lib_proto::EmployeeEmergencyInformation {
            id: self.id.to_string(),
            employee_id: self.employee_id.to_string(),
            address: self.address.unwrap_or_default(),
            tel_number: self.tel_number,
            mobile_number: self.mobile_number,
            contact_name: self.contact_name,
            health_conditions: self.health_conditions.unwrap_or_default(),
        }
    }
}

// storage file
impl Into<lib_proto::FileMetadata> for file::Model {
    fn into(self) -> lib_proto::FileMetadata {
        FileMetadata {
            id: self.id.to_string(),
            name: self.name,
            r#type: self.r#type,
            size: self.size as u32,
            is_public: self.is_public,
            owner_id: self.owner_id.to_string(),
        }
    }
}

impl Into<lib_proto::Employee> for employee::Model {
    fn into(self) -> lib_proto::Employee {
        lib_proto::management::Employee {
            id: self.id.to_string(),
            auth_user_id: self.auth_user_id.unwrap().to_string(),
            avatar_photo: None,
            cover_photo: None,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            tel_number: self.tel_number,
            mobile_number: self.mobile_number,
            email: self.email.unwrap(),
            role: self.role as i32,
            status: self.status as i32,
            contract_type: self.contract_type as i32,
            phil_nat_id: self.phil_nat_id,
            birth_date: self.birth_date.to_string(),
            special_interests: self.special_interests.unwrap_or_default(),
            learning_institutions: self.learning_institutions,
            spouse_first_name: self.spouse_first_name,
            spouse_middle_name: self.spouse_middle_name,
            spouse_last_name: self.spouse_last_name,
            spouse_employer: self.spouse_employer,
        };
        unimplemented!()
    }
}
