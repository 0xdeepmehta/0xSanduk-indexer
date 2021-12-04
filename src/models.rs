use crate::diesel::ExpressionMethods;
use borsh::{BorshDeserialize, BorshSerialize};
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::Serialize;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::pubkey::Pubkey;
use crate::schema::sanduks;
use std::convert::TryFrom;


// used to get the data form the PDA Accounts
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SandukData {
    pub end_time: UnixTimestamp,
    pub receiver: Pubkey,
    pub amount: u64,
    pub sender: Pubkey,
}

// { end_time: 1638469792, receiver: Ev4gFVxZAWwt8XsyUpNsFHLp6HcKAvHG4rvu6PJCr59t, amount: 1000000000, sender: 8bqg5cakaDP88XRcNzBxngcpX9tTuK1XRHxg9iF35EEx }

// This struct is going to save in our database
#[derive(Queryable, Insertable, Serialize)]
#[table_name = "sanduks"]
pub struct Sanduk {
    pub pda_account: String,
    pub end_time: i64,
    pub receiver: String,
    pub amount: i64,
    pub sender: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct DataLength {
  pub length: u32,
}


impl Sanduk {
    // create a new sanduk
    pub fn new(pda_pubkey: String, pda_data: &Vec<u8>) -> Option<Self> {
        let offset: usize = 4;

        let data_length = DataLength::try_from_slice(&pda_data[..offset]).unwrap();

        let length = usize::try_from(data_length.length + u32::try_from(offset).unwrap()).unwrap();

        let a = SandukData::try_from_slice(&pda_data[offset..length]);
        println!("AHAH :: {:?}", a);

        let sanduk_data = match SandukData::try_from_slice(&pda_data[offset..length]) {
            Ok(a) => a,
            Err(e) => {
                println!("Failed to deserialize {} with error {:?}", pda_pubkey.to_string(), e);
                return None;
            }
        };
        println!("san :: {:?}", sanduk_data);

        Some(Sanduk {
            sender: sanduk_data.sender.to_string(),
            end_time: sanduk_data.end_time,
            receiver: sanduk_data.receiver.to_string(),
            amount: sanduk_data.amount as i64,
            pda_account: pda_pubkey,
        })
    }

    // function to get all the sanduks with the sender equal to the given public key.
    pub fn get_all_with_sender(pubkey: &String, conn: &PgConnection) -> Vec<Sanduk> {
        use crate::schema::sanduks::dsl::*;
        sanduks
            .filter(sender.eq(pubkey))
            .load::<Sanduk>(conn)
            .unwrap()
    }

    // function to get all the sanduks with the receiver equal to the given public key.
    pub fn get_all_with_receiver(pubkey: &String, conn: &PgConnection) -> Vec<Sanduk> {
        use crate::schema::sanduks::dsl::*;
        sanduks
            .filter(receiver.eq(pubkey))
            .load::<Sanduk>(conn)
            .unwrap()
    }

    // function to check if the database contains the particular id.
    fn id_is_present(id: &String, connection: &PgConnection) -> bool {
        use crate::schema::sanduks::dsl::*;
        match sanduks.find(id).first::<Sanduk>(connection) {
            Ok(_s) => true,
            _ => false,
        }
    }
    // function to insert a new Sanduk if the id is not present which we can check with id_is_present function and update if it is present.
    pub fn insert_or_update(sanduk: Sanduk, conn: &PgConnection) -> bool {
        
        if Sanduk::id_is_present(&sanduk.pda_account, conn) {
            use crate::schema::sanduks::dsl::{
                amount as a, end_time as e_t, pda_account as p_a, receiver as r, sender as s, sanduks,
            };
            diesel::update(sanduks.filter(p_a.eq(sanduk.pda_account)))
                .set((
                    a.eq(sanduk.amount),
                    r.eq(sanduk.receiver),
                    s.eq(sanduk.sender),
                    e_t.eq(sanduk.end_time),
                ))
                .execute(conn)
                .is_ok()
        } else {
            diesel::insert_into(crate::schema::sanduks::table)
                .values(&sanduk)
                .execute(conn)
                .is_ok()            
        }
    }
    
}