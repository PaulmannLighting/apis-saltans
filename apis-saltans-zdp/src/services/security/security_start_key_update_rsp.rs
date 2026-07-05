crate::zdp_command! {
    /// Security Start Key Update Response.
    derive { Copy }
    SecurityStartKeyUpdateRsp => Security_Start_Key_Update_rsp;
    cluster_id: 0x8045;
    group: Security;
    fields {
        status: u8,
    }
    getters {
    }
}
