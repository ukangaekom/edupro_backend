use sqlx::{PgPool, FromRow,Row,types::Uuid};
use crate::models::user::{input::*, output::*};




pub async fn get_user_account_details(user_id: Uuid, pool:&PgPool)-> Result<UserAccountDetails,sqlx::Error> {

        let user_details = sqlx::query_as::<_, UserAccountDetails>(r#"
        SELECT
            u.firstname,
            u.lastname,
            u.email,
            COALESCE(u.username, '') AS username,
            COALESCE(ur.total_xp, 0) AS total_xps,
            COALESCE(rnk.rank, 0) AS rank,
            COALESCE(sess.exam_count, 0) AS total_exams_taken,
            COALESCE(sess.practice_count, 0) AS total_practices_taken
        FROM users u
        LEFT JOIN user_rank ur
            ON ur.user_id = u.id
        LEFT JOIN (
            SELECT
                s.user_id,
                COUNT(*) FILTER (WHERE s.mode = 2) AS exam_count,
                COUNT(*) FILTER (WHERE s.mode = 1) AS practice_count
            FROM exam_sessions s
            GROUP BY s.user_id
        ) sess
            ON sess.user_id = u.id
        LEFT JOIN (
            SELECT
                u.id AS user_id,
                rank() OVER (ORDER BY COALESCE(ur.total_xp, 0) DESC NULLS LAST) AS rank
            FROM users u
            LEFT JOIN user_rank ur
                ON ur.user_id = u.id
        ) rnk
            ON rnk.user_id = u.id
        WHERE u.id = $1
    "#)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
        
    
    return Ok(user_details)  
}


fn get_registered_exam(){
    todo!()
}


fn get_taken_exams(){
    todo!()
}


fn get_exam_scores(){
    todo!()
}



fn get_practice_scores(){
    todo!()
}


fn get_user_leaderboard(){
    todo!()
}

fn get_exam_analytics(){
    todo!()
}


fn get_user_xp(){
    todo!()
}