use juniper::{GraphQLObject, GraphQLEnum};

#[derive(GraphQLObject)]
pub struct AccountGroup {
    id: String,
    name: String,
    account_type: AccountType,
    accounts: Vec<Account>,
}

#[derive(GraphQLEnum)]
pub enum AccountType {
    Debit,
    Credit,
}

#[derive(GraphQLObject)]
#[graphql(description = "A ledger")]
pub struct Ledger {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub created_at: String,
    pub accounts_groups: Vec<AccountGroup>,
    pub transactions: Vec<Transaction>,
}

#[derive(GraphQLObject)]
pub struct Account {
    id: String,
    name: String,
    unit: Unit,
    created_at: String,
}

#[derive(GraphQLObject)]
pub struct Unit {
    id: String,
    name: String,
}

#[derive(GraphQLObject)]
pub struct Transaction {
    credit: String,
    debit: String,
    credit_amount: i32,
    debit_amount: i32,
    date: String,
}
