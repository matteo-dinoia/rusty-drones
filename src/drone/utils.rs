use crate::drone::RustyDrone;
use rand::Rng;
use wg_2024::network::NodeId;
use wg_2024::packet::{Nack, NackType, Packet, PacketType};

impl RustyDrone{
    pub(super) fn should_drop(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..1.0) < self.pdr
    }

    pub(super) fn create_nack(
        &self,
        packet: Packet,
        nack_type: NackType,
        droppable: bool,
        is_shortcuttable: bool,
    ) -> Option<Packet> {
        if !droppable {
            if is_shortcuttable {
                self.use_shortcut(packet);
            }
            return None;
        }

        let mut reversed_routes = packet.routing_header.sub_route(0..=packet.routing_header.hop_index)?;
        reversed_routes.reverse();
        reversed_routes.hop_index = 1;

        Some(Packet::new_nack(
            reversed_routes,
            packet.session_id,
            Nack {
                nack_type,
                fragment_index: get_fragment_index(packet.pack_type),
            },
        ))
    }

    pub(super) fn already_received_flood(
        &self,
        flood_id: u64,
        initiator_id: NodeId,
        _session_id: u64,
    ) -> bool {
        // Should keep in mind all of them but will only use flood_id as per protol
        // this is broken and wont work
        // so we will see what to do
        // TODO talk with WG
        self.received_floods.contains(&(flood_id, initiator_id))
    }
}


pub(super) fn get_fragment_index(packet_type: PacketType) -> u64 {
    if let PacketType::MsgFragment(f) = packet_type {
        return f.fragment_index;
    }
    0
}