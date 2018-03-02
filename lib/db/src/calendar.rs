//CREATE TABLE IF NOT EXISTS `cal` (
//`id` bigint(20) NOT NULL,
//`datum` date DEFAULT NULL,
//`dag_type` varchar(1) DEFAULT NULL,
//`type` varchar(1) DEFAULT NULL,
//`title` varchar(255) DEFAULT NULL,
//`omschrijving` varchar(255) DEFAULT NULL,
//`optlock` bigint(20) NOT NULL DEFAULT '0'
//) ENGINE=InnoDB AUTO_INCREMENT=47 DEFAULT CHARSET=latin1;
//
//CREATE TABLE IF NOT EXISTS `opgaven` (
//`id` bigint(20) NOT NULL,
//`user_id` bigint(20) DEFAULT NULL,
//`calendar_id` bigint(20) DEFAULT NULL,
//`aangemeld` tinyint(1) DEFAULT '0',
//`optlock` bigint(20) NOT NULL DEFAULT '0'
//) ENGINE=InnoDB AUTO_INCREMENT=2120 DEFAULT CHARSET=latin1;


enum DagType {
    Ochtend,
    Middag,
    Avond,
    Nacht
}

enum Ztype {

}

struct Jaar {
    pub id: i64,
    pub datum: NaiveDate,



    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
