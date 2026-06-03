import unittest
from twin_wallet_1047 import TwinWalletSubstrate
from twin_wallet_bridge import TwinWalletBridge

class TestTwinWalletSubstrate(unittest.TestCase):
    def setUp(self):
        self.substrate = TwinWalletSubstrate()
        self.bridge = TwinWalletBridge("http://localhost:8545")

    def test_substrate_metadata(self):
        self.assertEqual(self.substrate.id, "1047")
        self.assertEqual(self.substrate.name, "TWIN_WALLET")
        self.assertEqual(self.substrate.category, "security")
        self.assertIn("954", self.substrate.cross_links())
        contracts = self.substrate.get_canonized_contracts()
        self.assertEqual(contracts["TwinFactory"], "0x260C074c3afDc46A209D4619B5FAdB2964dF9a28")
        self.assertEqual(contracts["TwitchJWTVerifier"], "0xBDfC552469f11843802BCD7ec9a8372c8020fee8")

    def test_wallet_derivation(self):
        address = self.bridge.get_wallet_address("user123", "salt1")
        self.assertTrue(address.startswith("0x"))
        self.assertEqual(len(address), 42)

    def test_jwt_verification(self):
        result = self.bridge.verify_jwt_onchain("header.payload.signature", "signature")
        self.assertTrue(result)

if __name__ == "__main__":
    unittest.main()
