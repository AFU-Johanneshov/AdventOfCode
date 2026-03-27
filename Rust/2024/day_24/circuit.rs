use std::collections::HashMap;

pub struct Circuit {
    input_x: Vec<usize>,
    input_y: Vec<usize>,
    component_ids: HashMap<String, usize>,
    components: Vec<CircuitComponent>,
    output_z: Vec<usize>,
}

#[derive(Clone, Debug)]
struct Wire {
    id: usize,
    source: usize,
    powered: bool,
    connections: Vec<usize>,
}

impl Wire {
    fn new(id: usize) -> Wire {
        Wire {
            id,
            source: 0,
            powered: false,
            connections: Vec::new(),
        }
    }

    fn shares_connections(&self, other: &Wire) -> Vec<usize> {
        let mut result = Vec::new();
        for id in &self.connections {
            if other.connections.contains(&id) {
                result.push(*id);
            }
        }
        result
    }

    fn exclusive_connections(&self, other: &Wire) -> Vec<usize> {
        let mut result = Vec::new();
        for id in &self.connections {
            if !other.connections.contains(id) {
                result.push(*id);
            }
        }
        for id in &other.connections {
            if !self.connections.contains(id) {
                result.push(*id);
            }
        }
        result
    }

    fn get_connected_gate(
        &self,
        circuit: &Circuit,
        desired_operation: GateOperation,
    ) -> Option<(Gate, usize)> {
        for connection_index in &self.connections {
            let gate = circuit.get_gate(*connection_index);
            if gate.operation == desired_operation {
                return Some((*gate, *connection_index));
            }
        }
        None
    }

    fn is_connected(&self, gate: &Gate) -> bool {
        self.connections.contains(&gate.id) || gate.output == self.id
    }
}

#[derive(Clone, Copy, Debug)]
struct Gate {
    id: usize,
    input_1: usize,
    input_2: usize,
    operation: GateOperation,
    output: usize,
    output_powered: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateOperation {
    XOR,
    OR,
    AND,
}

impl GateOperation {
    fn parse(str: &str) -> Option<GateOperation> {
        match str {
            "XOR" => Some(Self::XOR),
            "OR" => Some(Self::OR),
            "AND" => Some(Self::AND),
            _ => None,
        }
    }
}

impl Gate {
    fn update(&mut self, circuit: &Circuit) {
        self.output_powered = match self.operation {
            GateOperation::OR => {
                circuit.components[self.input_1].output()
                    || circuit.components[self.input_2].output()
            }
            GateOperation::XOR => {
                circuit.components[self.input_1].output()
                    != circuit.components[self.input_2].output()
            }
            GateOperation::AND => {
                circuit.components[self.input_1].output()
                    && circuit.components[self.input_2].output()
            }
        }
    }

    fn other_input(&self, input: usize) -> usize {
        if self.input_1 == input {
            self.input_2
        } else {
            self.input_1
        }
    }
}

#[derive(Clone, Debug)]
pub enum CircuitComponent {
    Wire(Wire),
    Gate(Gate),
}

impl CircuitComponent {
    fn output(&self) -> bool {
        match self {
            CircuitComponent::Wire(wire) => wire.powered,
            CircuitComponent::Gate(gate) => gate.output_powered,
        }
    }
}

struct FullAdder {
    xor_1: Gate,
    and_1: Gate,
    xor_2: Vec<Gate>,
    and_2: Vec<Gate>,
    or: Vec<Gate>,
}

impl FullAdder {
    fn new(xor_1: Gate, and_1: Gate) -> FullAdder {
        FullAdder {
            xor_1,
            and_1,
            xor_2: Vec::new(),
            and_2: Vec::new(),
            or: Vec::new(),
        }
    }
}

// TODO:
// Store the components as objects in an array/vec. Use indexes as pointers to other components.
// Any changes to the circuit must be done through specific methods.
// Most processing will be done in the impl Circuit block.
// Wire, CircuitComponent, Gate, will be mostly useless outside the context within a Circuit. Same with
// the update functions.
// Updating the components will be done by the circuit as it requires mutable access to the
// connected components, which wouldn't be logical to force upon the components themselves. Let
// them be basic data holders, and let the collection of components actually do the processing.

impl Circuit {
    pub fn new() -> Circuit {
        todo!();
    }

    pub fn set_input(&mut self, modifier: char, index: usize, powered: bool) {
        //println!("Circuit: \nCircuitComponents: {:?}", self.components);
        let wire_index = match modifier {
            'x' => self.input_x[index],
            'y' => self.input_y[index],
            _ => panic!(
                "The input modifier {} is not valid. Please use 'x' or 'y'",
                modifier
            ),
        };
        let CircuitComponent::Wire(mut wire) = self.get_component(wire_index) else {
            panic!("The input wire for id {}{} residing at index {} in components didn't return a wire.", modifier, index, wire_index);
        };

        wire.powered = powered;
        self.set_component(wire_index, CircuitComponent::Wire(wire));
        self.update_connected_components(wire_index);
    }

    fn get_component(&self, index: usize) -> CircuitComponent {
        self.components[index].clone()
    }

    fn get_wire(&self, index: usize) -> &Wire {
        if let CircuitComponent::Wire(wire) = &self.components[index] {
            return wire;
        }
        panic!("CircuitComponent at index: {} was not a wire!", index);
    }

    fn get_gate(&self, index: usize) -> &Gate {
        if let CircuitComponent::Gate(gate) = &self.components[index] {
            return gate;
        }
        panic!("CircuitComponent at index: {} was not a gate!", index);
    }

    fn set_component(&mut self, index: usize, component: CircuitComponent) {
        self.components[index] = component;
    }

    fn update_connected_components(&mut self, component_index: usize) {
        let mut update_list: Vec<usize> = Vec::new();

        match &self.components[component_index] {
            CircuitComponent::Wire(wire) => update_list = wire.connections.clone(),
            CircuitComponent::Gate(gate) => update_list.push(gate.output),
        }

        while !update_list.is_empty() {
            let mut update_list_cache: Vec<usize> = Vec::new();
            for component_index in update_list {
                if let Some(mut further_updates) = self.update_component(component_index) {
                    update_list_cache.append(&mut further_updates);
                }
            }
            update_list = update_list_cache;
        }
    }

    fn update_component(&mut self, component_index: usize) -> Option<Vec<usize>> {
        match self.get_component(component_index) {
            CircuitComponent::Gate(gate) => self.update_gate(component_index, gate),
            CircuitComponent::Wire(wire) => self.update_wire(component_index, wire),
        }
    }

    fn update_wire(&mut self, index: usize, mut wire: Wire) -> Option<Vec<usize>> {
        if self.components[wire.source].output() == wire.powered {
            // The wire already has the new value so no new updates are neccesary.
            return None;
        }

        let further_updates = wire.connections.clone();
        wire.powered = self.components[wire.source].output();
        self.set_component(index, CircuitComponent::Wire(wire));
        Some(further_updates)
    }

    fn update_gate(&mut self, index: usize, mut gate: Gate) -> Option<Vec<usize>> {
        let current_output = gate.output_powered;
        gate.update(&self);
        if current_output == gate.output_powered {
            // The gate output has not changed so no new updates are neccesary.
            return None;
        }

        let further_updates = vec![gate.output];
        self.set_component(index, CircuitComponent::Gate(gate));
        Some(further_updates)
    }

    pub fn output(&self) -> Vec<bool> {
        let mut binary = Vec::new();
        for index in &self.output_z {
            if let CircuitComponent::Wire(wire) = self.get_component(*index) {
                binary.push(wire.powered);
            }
        }

        binary
    }

    fn get_component_name(&self, id: usize) -> String {
        for (name, component_id) in &self.component_ids {
            if *component_id == id {
                return name.clone();
            }
        }
        panic!("id: {} does not exist!", id)
    }

    fn swap_wires(&mut self, wire_1_id: usize, wire_2_id: usize) {
        let (mut wire_1, mut wire_2) = (
            self.get_wire(wire_1_id).clone(),
            self.get_wire(wire_2_id).clone(),
        );
        let (gate_1_id, gate_2_id) = (wire_1.source, wire_2.source);
        let (mut gate_1, mut gate_2) = (
            self.get_gate(wire_1.source).clone(),
            self.get_gate(wire_2.source).clone(),
        );
        gate_1.output = wire_2_id;
        gate_2.output = wire_1_id;
        wire_1.source = gate_2_id;
        wire_2.source = gate_1_id;
        self.set_component(wire_1_id, CircuitComponent::Wire(wire_1));
        self.set_component(wire_2_id, CircuitComponent::Wire(wire_2));
        self.set_component(gate_1_id, CircuitComponent::Gate(gate_1));
        self.set_component(gate_2_id, CircuitComponent::Gate(gate_2));

        println!(
            "Swapped: {} with {}",
            self.get_component_name(wire_1_id),
            self.get_component_name(wire_2_id)
        );
    }

    pub fn fix(&mut self) {
        let output_0_wire = self.get_wire(self.output_z[0]);
        let (xor_1, xor_1_id) = self
            .get_wire(self.input_x[0])
            .get_connected_gate(self, GateOperation::XOR)
            .unwrap();
        if output_0_wire.source != xor_1_id {
            println!("z0 was not connected to input1xor and has been swapped.");
            self.swap_wires(self.output_z[0], xor_1.output);
        }
        let mut carry_id = self
            .get_wire(self.input_x[0])
            .get_connected_gate(self, GateOperation::AND)
            .unwrap()
            .0
            .output;
        let mut carry = self.get_wire(carry_id).clone();

        for i in 1..self.input_x.len() {
            let ((xor_1, xor_1_id), (and_1, and_1_id)) = (
                self.get_wire(self.input_x[i])
                    .get_connected_gate(self, GateOperation::XOR)
                    .unwrap(),
                self.get_wire(self.input_x[i])
                    .get_connected_gate(self, GateOperation::AND)
                    .unwrap(),
            );
            let (xor_1_output_wire, and_1_output_wire) = (
                self.get_wire(xor_1.output).clone(),
                self.get_wire(and_1.output).clone(),
            );

            if !xor_1_output_wire.exclusive_connections(&carry).is_empty() {
                if let Some((xor_2, xor_2_id)) =
                    xor_1_output_wire.get_connected_gate(self, GateOperation::XOR)
                {
                    if xor_2.output == self.output_z[i] {
                        println!("carry is likely connected to the wrong source.");
                        self.swap_wires(carry_id, {
                            if xor_2.input_1 == xor_1.output {
                                xor_2.input_2
                            } else {
                                xor_2.input_1
                            }
                        });
                    }
                } else if let Some((xor_2, xor_2_id)) =
                    carry.get_connected_gate(self, GateOperation::XOR)
                {
                    if xor_2.output == self.output_z[i] {
                        println!("xor_1 output is likely wrong.");
                        self.swap_wires(xor_1.output, {
                            if xor_2.input_1 == carry_id {
                                xor_2.input_2
                            } else {
                                xor_2.input_1
                            }
                        });
                    }
                };
            } else {
                let (xor_2, xor_2_id) = xor_1_output_wire
                    .get_connected_gate(self, GateOperation::XOR)
                    .unwrap();
                if xor_2.output != self.output_z[i] {
                    println!("Swapped: xor_2.output with output_z[i]");
                    self.swap_wires(xor_2.output, self.output_z[i]);
                }
            }

            let (xor_1, xor_1_id) = self
                .get_wire(self.input_x[i])
                .get_connected_gate(self, GateOperation::XOR)
                .unwrap();

            let and_1_output_wire = self
                .get_wire(
                    self.get_wire(self.input_x[i])
                        .get_connected_gate(self, GateOperation::AND)
                        .unwrap()
                        .0
                        .output,
                )
                .clone();
            let and_2_output_wire = self.get_wire(
                self.get_wire(
                    self.get_wire(self.input_x[i])
                        .get_connected_gate(self, GateOperation::XOR)
                        .unwrap()
                        .0
                        .output,
                )
                .get_connected_gate(self, GateOperation::AND)
                .unwrap()
                .0
                .output,
            );

            let Some((and_2, and_2_id)) = self
                .get_wire(xor_1.output)
                .get_connected_gate(self, GateOperation::AND)
            else {
                panic!(
                    "Failed to get a AND gate connected to wire with id: {}\nWire connects to {} elements.",
                    xor_1.output, self.get_wire(xor_1.output).connections.len()
                );
            };
            let and_2_output_wire = self.get_wire(and_2.output).clone();

            if !and_1_output_wire
                .exclusive_connections(&and_2_output_wire)
                .is_empty()
            {
                // and_1 & and_2 outputs are not connected to the same gate!
                // One or both of them are wrongly connected.

                // for or_gate in or_gates connected to the two and gates.
                for gate_id in and_1_output_wire.exclusive_connections(&and_2_output_wire) {
                    let gate = self.get_gate(gate_id);
                    if gate.operation == GateOperation::OR {
                    } else {
                        // the wire connected to this gate is wrongly connected!
                        // Figure out the correct gate by:
                        // 1: get connected OR gate from the output_wire not connected to gate.
                        // 2: check validity of OR gate output. (Does it connect to a xor connected
                        //    to output_z[i+1])
                        // 3: If yes then swap gate.source with other OR gates input wire which is
                        //    NOT connected to the other AND gate.

                        let other_or_gate = {
                            if and_1_output_wire.is_connected(gate) {
                                and_2_output_wire.get_connected_gate(self, GateOperation::OR)
                            } else {
                                and_1_output_wire.get_connected_gate(self, GateOperation::OR)
                            }
                        };

                        let Some((other_or_gate, _)) = other_or_gate else {
                            // Neither of the two AND gates connect to a OR gate.
                            panic!(
                                "Found two AND gates with outputs NOT connected to any OR gate."
                            );
                        };

                        let Some((xor, _)) = self
                            .get_wire(other_or_gate.output)
                            .get_connected_gate(self, GateOperation::XOR)
                        else {
                            // other_or_gate is not connected to a xor gate!
                            if i == self.input_x.len() - 1 {
                                if other_or_gate.output == self.output_z[self.output_z.len() - 1] {
                                    // OR gate is valid!
                                    if and_1_output_wire.is_connected(gate) {
                                        self.swap_wires(
                                            and_1_output_wire.id,
                                            other_or_gate.other_input(and_2_output_wire.id),
                                        );
                                        continue;
                                    }
                                }
                            }
                            panic!("other_or_gate is not connected to a xor gate!");
                        };

                        if xor.output == self.output_z[i + 1] {
                            // OR gate is valid!
                            if i == self.input_x.len() - 1 {
                                if other_or_gate.output == self.output_z[self.output_z.len() - 1] {
                                    // OR gate is valid!
                                    if and_1_output_wire.is_connected(gate) {
                                        self.swap_wires(
                                            and_1_output_wire.id,
                                            other_or_gate.other_input(and_2_output_wire.id),
                                        );
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Both and gates share the same target gate/s with their outputs.
                let Some(or_gate) = and_1_output_wire.get_connected_gate(self, GateOperation::OR)
                else {
                    panic!("No or_gate found connected to and_1!");
                };
                carry_id = or_gate.0.output;
                carry = self.get_wire(carry_id).clone();
                //println!("Carry: {}", carry_id);

                // TODO: The output of or_gate is checked in the next iteration of the loop.
                // But what about the last iteration where carry should instead connect to the last
                // output_z?
            }
        }

        println!("Fixed!");
    }

    pub fn fix_FullAdders(&mut self) {
        let carry_wire: Option<Wire> = None;
        let mut faulty_wires: Vec<usize> = Vec::new();
        for index in 0..self.input_x.len() {
            let (x_input_wire, y_input_wire, z_output_wire) = (
                self.get_wire(self.input_x[index]),
                self.get_wire(self.input_y[index]),
                self.get_wire(self.output_z[index]),
            );

            let (first_xor, first_and) = {
                if !(x_input_wire.connections[0] == y_input_wire.connections[0]
                    && x_input_wire.connections[1] == y_input_wire.connections[1])
                {
                    panic!("Input wire connection for wires index: {} has a mismatch! This should not be possible based on the rules specified.", index);
                }

                let (gate_1, gate_2) = (
                    self.get_gate(x_input_wire.connections[0]),
                    self.get_gate(x_input_wire.connections[1]),
                );

                if gate_1.operation == GateOperation::XOR && gate_2.operation == GateOperation::AND
                {
                    (gate_1, gate_2)
                } else if gate_1.operation == GateOperation::AND
                    && gate_2.operation == GateOperation::XOR
                {
                    (gate_2, gate_1)
                } else {
                    panic!("Input wire connection for wires index: {} does not have the correct gate types connected!", index);
                }
            };
            let mut full_adder = FullAdder::new(*first_xor, *first_and);

            let (second_xor, second_and) = {
                let other_wire = carry_wire.clone();
                let first_xor_output = self.get_wire(first_xor.output);

                // If the wire doesn't have 2 connections it is guaranteed to be faulty.
                if first_xor_output.connections.len() != 2 {
                    faulty_wires.push(first_xor.output);
                    // TODO: Figure out the second XOR or AND and add the XOR input to faulty
                    // wires.
                }

                let second_xor = {
                    let mut result;
                    for (gate, id) in first_xor_output
                        .connections
                        .iter()
                        .map(|connection| (self.get_gate(*connection), connection))
                    {
                        if gate.operation == GateOperation::XOR {
                            if let Some(other_wire) = &other_wire {
                                if other_wire.connections.contains(id)
                                    || gate.output == self.output_z[index]
                                {
                                    result = gate;
                                    break;
                                }
                            }
                        }
                    }

                    5
                };

                if let Some(carry_wire) = &carry_wire {
                    for gate in first_xor_output.shares_connections(carry_wire) {}
                }
                let (gate_1, gate_2) = (
                    self.get_gate(first_xor_output.connections[0]),
                    self.get_gate(first_xor_output.connections[1]),
                );

                (5, 5)
            };
        }
    }

    // Rules:
    // AND gates sources must always be either:
    //
    // A & B s 1
    // A & B s 4
    // 1.o & C s 2
    // 1.o & C s 3
    // 2.o & 3.o s 5
    // 2.o s Z[index]
    // if last added:
    // 5.o s Z[index+1];
    //
    // let carry_wire = z[46];
    // For i in 45..0
    // if let OR_gate | carry_wire.source == OR
    //      if OR_gate.input_1.source != AND
    //          OR_gate.input_1 is wrongly connected!
    //      if OR_gate.input_2.source != AND
    //          OR_gate.input_2 is wrongly connected!
    //

    // #########################################################################################

    //  The first adder is not full. It should only consist of 1 AND & 1 XOR gate. Since there is
    //  no need for a carry in.
    //  Handle it first, then start processing the rest.
    //
    //
    //  if output_z[0].source != input_1.connections.XOR (xor)
    //      output_z[0] is wrongly connected.
    //      xor.output is wrongly connected.
    //      repair circuit by swapping the two.
    //
    //  carries = input_1.connections.AND.output
    //
    //  for i in 1..input_wires.len()
    //  {
    //  let xor_1, and_1 = gates connected to input wires.
    //
    //  for carry in carries
    //      if xor_1.output.connections != carry.shares_connections
    //          // xor_1.output or carry is wrongly connected.
    //          get any connected XOR gates to xor_1.output and carry
    //          for gate in XOR gates
    //              if gate.output == output_z[i]
    //                  // The wire NOT connected to this gate is wrongly connected. Meaning the wire
    //                  connected to this gate that is NOT connected to xor_1.output/carry is also
    //                  wrongly connected.
    //                  repair circuit by swapping the sources of the two wires.
    //                  break loop.
    //
    //      if xor_1.output and carry shares a connection with the same XOR gate (xor_2)
    //          if xor_2 is NOT connected to output_z[i]
    //              xor_2.output is wrongly connected.
    //              output_z[i] is wrongly connected.
    //              repair circuit by swapping the sources of xor_2.output and output_z[i]
    //
    //  4 out of five gates in this full_adder is now verified.
    //  Both input gates since they can't have been wrongly connected.
    //  The two other xor and and gates have at this point been fixed if they where broken.
    //      (xor_2, and_2)
    //
    //  if and_2.output != and_1.output
    //      one or both of them are wrongly connected.
    //      for or_gate in or_gates connected to and_1 & and_2
    //          if (
    //              (if i+1 >= input.len()) or_gate.output = output_z[i+2] ||
    //              or_gate.output.connections == input_x[i+1] ||
    //              or_gate.output.connections.XOR.output == output_z[i+1]
    //          )
    //              The wire NOT connected to this or_gate is wrongly connected.
    //              The wire connected to this gate that does not have one of the and gates as its
    //              source is wrongly connected.
    //              repair the circuit by swapping the two wires sources.
    //      if loop ends without a repair being performed panic because of the found fault without
    //      a clear solution.
    //
    //  if and_1 & and_2 shares a connection to the same OR gate (or_gate)
    //      set carry to or_gate.output
    //
    //
    //
    //
    //
    //
    //      if or_gate.output != carry (
    //      )
    //          or_gate.output is wrongly connected.
    //          carry is wrongly connected.
    //          repair the circuit by swapping the two wire sources.
    //
    //  }

    // #########################################################################################

    // carry = output_z[output_z.len()]
    // for i i input_wires.len()..0
    // {
    //  let xor_1, and_1 = gates connected to input wires.
    //  if and_1.output is connected to a OR gate (or_gate)
    //      if or_gate.output == carry
    //          and_1.output and or_gate.output are likely valid.
    //      else if (
    //          other input of or_gate is AND gate (other_and) &&
    //          (
    //              other_and.input_1.source is xor_1 ||
    //              other_and.input_1.connections.XOR.output == output_z[i]
    //          )
    //      ) ->
    //          or_gate.output is wrongly connected.
    //          carry.source is wrongly connected.
    //          repair the circuit by swapping the two wire sources. (And the connected gates
    //          outputs.)
    //
    //  if carry.source is a OR gate (or_gate)
    //      if or_gate.input_2 ==
    //  else
    //      // carry is wrongly connected. Figure out what or gate it is supposed to be connected
    //      to.
    // }
    //

    pub fn print_info(&self) {
        let (mut or, mut and, mut xor) = (0, 0, 0);
        for (index, component) in self.components.iter().enumerate() {
            match component {
                CircuitComponent::Wire(wire) => {
                    if wire.connections.len() > 2 {
                        panic!(
                            "Wire has more connections than the expected 2: \n{:?}",
                            wire
                        );
                    }
                    if (wire.source == 0 || wire.connections.is_empty()) {
                        println!(
                            "Loose wire: {} | {:?}",
                            self.get_component_name(index),
                            wire
                        );
                    }
                }
                CircuitComponent::Gate(gate) => match gate.operation {
                    GateOperation::OR => or += 1,
                    GateOperation::AND => and += 1,
                    GateOperation::XOR => xor += 1,
                },
            }
        }

        println!("Gate count: \nAND: {}\nXOR: {}\nOR: {}", and, xor, or);
    }
}

pub struct CircuitBuilder {
    component_ids: HashMap<String, usize>,
    components: Vec<CircuitComponent>,
}

impl CircuitBuilder {
    pub fn new() -> CircuitBuilder {
        CircuitBuilder {
            component_ids: HashMap::new(),
            components: Vec::new(),
        }
    }

    fn wire_exists(&self, wire_id: String) -> bool {
        self.component_ids.contains_key(&wire_id)
    }

    fn get_wire_index(&mut self, wire_id: String) -> usize {
        if let Some(index) = self.component_ids.get(&wire_id) {
            *index
        } else {
            self.components
                .push(CircuitComponent::Wire(Wire::new(self.components.len())));
            self.component_ids
                .insert(wire_id, self.components.len() - 1);
            self.components.len() - 1
        }
    }

    pub fn add_gate(
        &mut self,
        input_1: String,
        gate_operation: String,
        input_2: String,
        output: String,
    ) {
        let Some(gate_op) = GateOperation::parse(&gate_operation) else {
            panic!(
                "INVALID GATE OPERATION: [{}] is not a valid gate!",
                gate_operation
            );
        };

        let input_1_index: usize = self.get_wire_index(input_1);
        let input_2_index: usize = self.get_wire_index(input_2);
        let output_index: usize = self.get_wire_index(output);
        let gate_index = self.components.len();

        let gate = Gate {
            id: gate_index,
            input_1: input_1_index,
            input_2: input_2_index,
            operation: gate_op,
            output_powered: false,
            output: output_index,
        };
        self.components.push(CircuitComponent::Gate(gate));
        let gate_index = self.components.len() - 1;

        if let CircuitComponent::Wire(wire) = &mut self.components[input_1_index] {
            wire.connections.push(gate_index);
        } else {
            panic!(
                "Expected a wire at index: {} but found a gate! \n{:?}",
                input_1_index, self.components
            );
        }
        if let CircuitComponent::Wire(wire) = &mut self.components[input_2_index] {
            wire.connections.push(gate_index);
        } else {
            panic!(
                "expected a wire at index: {} but found a gate! \n{:?}",
                input_2_index, self.components
            );
        }

        if let CircuitComponent::Wire(wire) = &mut self.components[output_index] {
            wire.source = gate_index;
        } else {
            panic!(
                "expected a wire at index: {} but found a gate! \n{:?}",
                output_index, self.components
            );
        }
    }

    fn collect_bus(&mut self, modifier: char) -> Vec<usize> {
        let mut id = format!("{}00", modifier);
        let mut iteration = 0;
        let mut indexes = Vec::new();
        while let Some(index) = self.component_ids.get(&id) {
            indexes.push(*index);
            iteration += 1;
            id = if iteration >= 10 {
                format!("{}{}", modifier, iteration)
            } else {
                format!("{}0{}", modifier, iteration)
            };
        }
        //println!("Indexes: {:?}", indexes);
        indexes
    }

    pub fn assemble(mut self) -> Circuit {
        let output_z = self.collect_bus('z');
        Circuit {
            input_x: self.collect_bus('x'),
            input_y: self.collect_bus('y'),
            component_ids: self.component_ids,
            components: self.components,
            output_z,
        }
    }
}
