from time import perf_counter
from typing import List, Sequence

import numpy as np

from genart.cloudscript.generator import random_particle
from genart.cloudscript.models import Particle, SuperChamber


class Simulation:
    def __init__(
        self,
        chamber: SuperChamber,
        particles: Sequence[Particle],
        time_modifier: float = 1.0,
    ):
        self.chamber: SuperChamber = chamber
        self.particles: Sequence[Particle] = particles
        self.time_modifier = time_modifier

        self.clock: float = 0.0
        self.time_passed: float = 0.0
        self.new_part_buffer: List[Particle] = []

    def start(self):
        self.clock = perf_counter()

    def step(self):
        now = perf_counter()
        tdelta = (now - self.clock) * self.time_modifier
        self.time_passed += tdelta
        self.clock = now

        for p in self.particles:
            if p.is_alive:
                self._update_particle(p, tdelta)
        if self.new_part_buffer:
            self.particles.extend(self.new_part_buffer)
            self.new_part_buffer = []

    def _update_particle(self, p: Particle, tdelta: float):
        # Update lifetime
        p.lifetime += tdelta
        if p.lifetime >= p.decays_after:
            p.is_alive = False

            # Decay into smaller particles
            if p.mass > 1:
                self.split_particle(p)
        else:
            # Find in which chamber this particle is:
            chamber = self.chamber.chamber_at(*p.position)

            # Magnetic component of Lorentz force:
            # 2D HACK: Since we assume the magnetic field is pointed straight at us (e.g. [0, 0, x]),
            # We can shortcut the cross-product:
            mag_force = p.total_charge * (
                chamber._magnet_vector * np.flipud(p.velocity)
            )

            # Apply force:
            # F = m.a, so a = F / m
            acceleration = mag_force / p.mass
            p.velocity += acceleration * tdelta

            # Friction:
            p.velocity *= 1.0 - (chamber.friction * tdelta)

            # Apply velocity:
            p.position += p.velocity * tdelta

    def split_particle(self, p: Particle):
        if p.mass == 1:
            return

        # Split the particle into all its "atoms":
        atoms = [1] * p.charges[0] + [0] * p.charges[1] + [-1] * p.charges[2]
        atoms = np.random.permutation(atoms)
        idx = 0

        while idx <= (len(atoms) - 1):
            max_mass = len(atoms) - 1
            new_mass = np.random.randint(1, max_mass) if max_mass > 1 else 1
            new_charge_symbols = atoms[idx : idx + new_mass]
            idx = idx + new_mass

            unique, counts = np.unique(new_charge_symbols, return_counts=True)
            counts = dict(zip(unique, counts))
            new_charges = [counts.get(1, 0), counts.get(0, 0), counts.get(-1, 0)]

            self.new_part_buffer.append(
                random_particle(np.copy(p.position), np.copy(p.velocity), new_charges)
            )
