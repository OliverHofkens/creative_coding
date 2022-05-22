from time import perf_counter
from typing import List, Sequence

import numpy as np
from numpy.random import Generator

from .generator import make_particle
from .models import BubbleChamber, Particle


class Simulation:
    def __init__(
        self,
        chamber: BubbleChamber,
        particles: Sequence[Particle],
        rng: Generator,
        time_modifier: float = 1.0,
    ):
        self.chamber: BubbleChamber = chamber
        self.particles: Sequence[Particle] = particles
        self.rng = rng
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
            # Magnetic component of Lorentz force:
            mag_force = p.total_charge * np.cross(
                p.velocity, self.chamber._magnet_vector
            )

            # Apply force:
            # F = m.a, so a = F / m
            acceleration = mag_force / p.mass
            p.velocity += acceleration * tdelta

            # Friction:
            p.velocity *= 1.0 - (self.chamber.friction * tdelta)

            # Apply velocity:
            p.position += p.velocity * tdelta

    def split_particle(self, p: Particle):
        if p.mass == 1:
            return

        i = 0
        for split in p.split_tree.parts:
            atoms = p.charges[i : i + split.count]
            i += split.count

            self.new_part_buffer.append(
                make_particle(
                    self.rng,
                    p.position.copy(),
                    p.velocity.copy(),
                    atoms,
                    split,
                )
            )
