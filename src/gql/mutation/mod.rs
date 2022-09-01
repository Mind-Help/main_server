use async_graphql::MergedObject;

use user_mutation::UserMutation;
use doctor_mutation::DoctorMutation;

mod doctor_mutation;
mod user_mutation;

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, DoctorMutation);
