"""
The MIT License (MIT)

Copyright (c) 2015 Kim Blomqvist

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
"""

import xml.etree.ElementTree as ET


class SvdElement():
    def __init__(self, element=None, defaults={}):
        self.init()
        if element:
            self.from_element(element, defaults)

    def __repr__(self):
        from pprint import pformat
        return pformat(vars(self), width=72, indent=4)

    def init(self):
        """Define object variables within this method"""
        raise NotImplementedError("Please implement")

    def from_element(self, element, defaults={}):
        """Populate object variables from element text/attrib"""
        try:
            defaults = vars(defaults)
        except: pass

        for key, default in vars(self).items():
            if isinstance(default, list) or isinstance(default, dict):
                continue
            try:
                setattr(self, key, element.find(key).text)
            except: # Maybe it's attribute?
                if key in defaults:
                    default = defaults[key]
                setattr(self, key, element.get(key, default))

    def inherit_from(self, element):
        for key, value in vars(self).items():
            if not value and key in vars(element):
                value = getattr(element, key)
                setattr(self, key, value)


class SvdDevice(SvdElement):
    def init(self):
        self.name = None
        self.version = None
        self.description = None
        self.addressUnitBits = None
        self.width = None
        self.size = None
        self.access = None
        self.resetValue = None
        self.resetMask = None
        self.vendor = None
        self.vendorID = None
        self.series = None
        self.licenseText = None
        self.headerSystemFilename = None
        self.headerDefinitionsPrefix = None


class SvdCpu(SvdElement):
    def init(self):
        self.name = None
        self.revision = None
        self.endian = None
        self.mpuPresent = None
        self.fpuPresent = None
        self.fpuDP = None
        self.icachePresent = None
        self.dcachePresent = None
        self.itcmPresent = None
        self.dtcmPresent = None
        self.vtorPresent = None
        self.nvicPrioBits = None
        self.vendorSystickConfig = None


class SvdPeripheral(SvdElement):
    def init(self):
        self.registers = []
        self.derivedFrom = None
        self.name = None
        self.version = None
        self.description = None
        self.groupName = None
        self.prependToName = None
        self.appendToName = None
        self.disableCondition = None
        self.baseAddress = None
        self.size = None
        self.access = None
        self.resetValue = None
        self.resetMask = None
        self.alternatePeripheral = None


class SvdRegister(SvdElement):
    def init(self):
        self.fields = []
        self.derivedFrom = None
        self.dim = None
        self.dimIncrement = None
        self.dimIndex = None
        self.name = None
        self.displayName = None
        self.description = None
        self.alternateGroup = None
        self.addressOffset = None
        self.size = None
        self.access = None
        self.resetValue = None
        self.resetMask = None
        self.modifiedWriteValues = None
        self.readAction = None
        self.alternateRegister = None
        self.dataType = None

    def from_element(self, element, defaults={}):
        SvdElement.from_element(self, element, defaults)

        try: # Because fields may be None
            for e in element.find("fields"):
                field = SvdField(e, self)
                self.fields.append(field)
        except: pass


class SvdCluster(SvdElement):
    def init(self):
        self.registers = []
        self.derivedFrom = None
        self.dim = None
        self.dimIncrement = None
        self.dimIndex = None
        self.name = None
        self.description = None
        self.alternateCluster = None
        self.headerStructName = None
        self.addressOffset = None

    def from_element(self, element, defaults={}):
        SvdElement.from_element(self, element, {})
        try:
            for e in element.findall("*"):
                if e.tag == "cluster": # Cluster may include yet another cluster
                    self.registers.append(SvdCluster(e, defaults))
                elif e.tag == "register":
                    self.registers.append(SvdRegister(e, defaults))
        except: pass

        # Normalize cluster name and dimension
        if not self.name.endswith("[%s]"):
            self.name = self.name + "[%s]"
            self.dim = 1


class SvdField(SvdElement):
    def init(self):
        self.enumeratedValues = {
            "read": [],
            "write": [],
            "read-write": [],
        }
        self.derivedFrom = None
        self.name = None
        self.description = None
        self.bitOffset = None
        self.bitWidth = None
        self.lsb = None
        self.msb = None
        self.bitRange = None
        self.access = None
        self.modifiedWriteValues = None
        self.writeConstraint = None
        self.readAction = None

    def from_element(self, element, defaults={}):
        SvdElement.from_element(self, element, defaults)

        if self.bitRange:
            self.msb, self.lsb = self.bitRange[1:-1].split(":")
        if self.msb and self.lsb:
            self.msb = int(self.msb)
            self.lsb = int(self.lsb)
        else:
            self.lsb = int(self.bitOffset)
            self.msb = int(self.bitWidth) + self.lsb
        self.bitOffset = self.lsb
        self.bitWidth = self.msb - self.lsb + 1
        self.bitRange = "[{}:{}]".format(self.msb, self.lsb)

        try: # Because enumeratedValues may be None
            for e in element.findall("enumeratedValues"):
                try:
                    usage = e.find("usage").text
                except:
                    usage = "read-write"
                for e in e.findall("enumeratedValue"):
                    enum = SvdEnumeratedValue(e, {})
                    self.enumeratedValues[usage].append(enum)
        except: pass


class SvdEnumeratedValue(SvdElement):
    def init(self):
        self.derivedFrom = None
        self.name = None
        self.description = None
        self.value = None
        self.isDefault = None


class SvdFile():
    def __init__(self, file):
        if type(file) is str:
            self.root = ET.fromstring(file)
        else:
            tree = ET.parse(file)
            self.root = tree.getroot()

    def parse(self):
        self.cpu = SvdCpu(self.root.find("cpu"))
        self.device = SvdDevice(self.root)

        self.peripherals = {}
        self.peripherals_order = []
        self.derived_peripherals = []
        self.peripheral_groups = {}

        for e in self.root.iter("peripheral"):
            p = SvdPeripheral(e, self.device)
            try: # Registers may be None
                for e in e.find("registers"):
                    if e.tag == "cluster":
                        p.registers.append(SvdCluster(e, p))
                    elif e.tag == "register":
                        p.registers.append(SvdRegister(e, p))
            except: pass

            if p.derivedFrom:
                self.derived_peripherals.append(p.name)

            self.peripherals[p.name] = p
            self.peripherals_order.append(p.name)

        for p in [self.peripherals[name] for name in self.derived_peripherals]:
            base = self.peripherals[p.derivedFrom]
            p.inherit_from(base)
