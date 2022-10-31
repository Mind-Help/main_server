use async_graphql::MergedObject;

use doctor_mutation::DoctorMutation;
use user_mutation::UserMutation;

mod doctor_mutation;
mod user_mutation;

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, DoctorMutation);
