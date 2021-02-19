use crate::models::schema::club;

#[derive(Debug, Insertable)]
#[table_name = "club"]
pub struct NewClubDto<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub is_published: bool,
}
