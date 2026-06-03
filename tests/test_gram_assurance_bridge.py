import pytest
import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'substrates', '1028-GRAM-ASSURANCE-BRIDGE')))

from gram_assurance_bridge_1028 import GramAssuranceBridge, LPRMValueHead
import numpy as np

def test_gram_assurance_bridge():
    bridge = GramAssuranceBridge(
        claim="A trajetória GRAM converge para solução válida com alta confiança",
        context="Sudoku-Extreme / N-Queens / ARC-AGI",
        lprm_dim=512
    )
    bridge.build_standard_structure()

    np.random.seed(42)
    trajectory = [np.random.randn(512) for _ in range(8)]
    zk_proof = "65576cc7d513fefd46210f58"

    result = bridge.evaluate_trajectory(trajectory, zk_proof)

    assert result["trajectory_id"] == 1
    assert result["safety_case_status"] in ["FAILED", "SATISFIED"]
    assert len(result["confidence_trajectory"]) == 8

    bundle = bridge.get_assurance_bundle()
    assert bundle["seal"] == "1028-GRAM-ASSURANCE-BRIDGE-2026-06-03"
