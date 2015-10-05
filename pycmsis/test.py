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

import svd
import pytest
import xml.etree.ElementTree as et

def test_register_folding_commasparated_index():
    r = svd.Register(et.fromstring(
        """
        <register>
            <dim>3</dim>
            <dimIncrement>4</dimIncrement>
            <dimIndex>A,B,C</dimIndex>
            <name>GPIO_%s</name>
            <addressOffset>4</addressOffset>
        </register>
        """
    ))
    a = r.to_array() # or maybe fold() would be more descriptive?

    assert len(a) == 3
    assert a[0].name == "GPIO_A"
    assert a[1].name == "GPIO_B"
    assert a[2].name == "GPIO_C"

def test_register_folding_integerrange_index():
    r = svd.Register(et.fromstring(
        """
        <register>
            <dim>4</dim>
            <dimIncrement>4</dimIncrement>
            <dimIndex>3-6</dimIndex>
            <name>IRQ%s</name>
            <addressOffset>4</addressOffset>
        </register>
        """
    ))
    a = r.to_array() # or maybe fold() would be more descriptive?

    assert len(a) == 4
    assert a[0].name == "IRQ3"
    assert a[1].name == "IRQ4"
    assert a[2].name == "IRQ5"
    assert a[3].name == "IRQ6"
    assert a[2].addressOffset == 4 + (2 * 4)
