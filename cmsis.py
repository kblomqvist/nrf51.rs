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

class SvdFile():
	device = {
		"name": None,
		"version": None,
		"description": None,
		#"addressUnitBits": None,
		"width": None,

		"size": None,
		"access": None,
		#"resetValue": None,
		#"resetMask": None,

		"vendor": None,
		"vendorID": None,
		"series": None,
		#"licenseText": None,
		#"headerSystemFilename": None,
		#"headerDefinitionsPrefix:" None,
	}

	cpu = {
		"name": None,
		"revision": None,
		"endian": None,
		"mpuPresent": None,
		"fpuPresent": None,
		"fpuDP": None,
		"icachePresent": None,
		"dcachePresent": None,
		"itcmPresent": None,
		"dtcmPresent": None,
		"vtorPresent": None,
		"nvicPrioBits": None,
		"vendorSystickConfig": None,
	}

	periphs = {}
	periphs_order = []
	derived_periphs = []
	peripheral_groups = {}

	def __init__(self, file):
		if type(file) is str:
			self.root = ET.fromstring(file)
		else:
			tree = ET.parse(file)
			self.root = tree.getroot()

	def populate(self, target, element, defaults={}):
		"""Populate target dictionary key values from element text/attrib"""
		for tag in target.keys():
			try:
				target[tag] = element.find(tag).text
			except:
				default = defaults[tag] if tag in defaults else target[tag]
				target[tag] = element.get(tag, default) # Maybe it's attribute

	def parse_interrupt(self, periph, element):
		if element:
			periph["interrupt"] = {
				"name": element.find("name").text,
				"value": element.find("value").text,
			}

	def parse_register(self, periph, element):
		register = {
			"derivedFrom": None,

			"dim": None,
			"dimIncrement": None,
			"dimIndex": None,

			"name": None,
			#"displayName": None,
			"description": None,
			#"alternateGroup": None,
			"addressOffset": None,

			"size": None,
			"access": None,
			#"resetValue": None,
			#"resetMask": None,

			#"modifiedWriteValues": None,
			#"readAction": None,
			
			#"alternateRegister": None,
			#"dataType": None,
		}
		self.populate(register, element, periph)

		if register["dim"]: # Is register array
			register["registers"] = [{
				"name": register["name"].replace("[%s]", ""),
				"addressOffset": "0x0",
				"size": register["size"],
				"access": register["access"],
			}]
			self.parse_fields(register["registers"][0], element.find("fields"))
			register["name"] = register["name"].replace("%s", register["dim"])
		else:
			self.parse_fields(register, element.find("fields"))

		return register

	def parse_cluster(self, periph, element):
		reg = self.parse_register(periph, element)
		reg["registers"] = []

		if not reg["dim"]:
			reg["name"] = reg["name"] + "[1]"
		else:
			reg["name"] = reg["name"].replace("%s", reg["dim"])

		for elem in element.findall("register"):
			reg["dim"] = None
			cluster_register = self.parse_register(reg, elem)
			reg["registers"].append(cluster_register)

		return reg

	def parse_registers(self, periph, element):
		periph["registers"] = []
		if not element:
			return

		for elem in element.findall("*"):
			if elem.tag == "register":
				reg = self.parse_register(periph, elem)
			elif elem.tag == "cluster":
				reg = self.parse_cluster(periph, elem)
			if "reg" in locals():
				periph["registers"].append(reg)

	def parse_field(self, register, element):
		field = {
			"derivedFrom": None, # Not supported
			"name": None,
			"description": None,

			"bitOffset": None,
			"bitWidth": None,
			"lsb": None,
			"msb": None,
			"bitRange": None,

			"access": None,
			"modifiedWriteValues": None,
			"writeConstraint": None,
			"readAction": None,
		}
		self.populate(field, element, register)

		if not field["bitRange"]:
			if field["bitOffset"]:
				lsb = int(field["bitOffset"])
				msb = int(field["bitWidth"]) + lsb
			else:
				lsb = int(field["lsb"])
				msb = int(field["msb"])
			field["bitRange"] = "[{}:{}]".format(msb, lsb)

		self.parse_enumvalues(field, element.find("enumeratedValues"))
		return field

	def parse_fields(self, register, element):
		register["fields"] = []
		if not element:
			return
		for elem in element.findall("field"):
			field = self.parse_field(register, elem)
			register["fields"].append(field)

	def parse_enumvalue(self, field, element):
		enum = {
			"derivedFrom": None, # Not supported really
			"name": None,
			"description": None,
			"value": None,
			"isDefault": None,
		}
		self.populate(enum, element, field)
		return enum

	def parse_enumvalues(self, field, element):
		field["enumeratedValues"] = []
		if not element:
			return
		for elem in element.findall("enumeratedValue"):
			enum = self.parse_enumvalue(field, elem)
			field["enumeratedValues"].append(enum)

	def parse(self):
		# Device and CPU
		self.populate(self.device, self.root)
		self.populate(self.cpu, self.root.find("cpu"))

		# Peripherals
		for element in self.root.iter("peripheral"):
			periph = {
				"derivedFrom": None,

				"name": None,
				"version": None,
				"description": None,
				#"groupName": None,
				#"prependToName": None,
				#"appendToName": None,
				#"disableCondition": None,
				"baseAddress": None,

				"size": None,
				"access": None,
				#"resetValue": None,
				#"resetMask": None,

				#"alternatePeripheral": None,
			}
			self.populate(periph, element, self.device)
			self.parse_interrupt(periph, element.find("interrupt"))
			self.parse_registers(periph, element.find("registers"))

			self.periphs[periph["name"]] = periph
			self.periphs_order.append(periph["name"])

			if "derivedFrom" in periph and periph["derivedFrom"]:
				self.derived_periphs.append(periph["name"])
			
			if "groupName" in periph and periph["groupName"]:
				try:
					self.peripheral_groups[periph["name"]].append(periph)
				except:
					self.peripheral_groups[periph["name"]] = [periph]
