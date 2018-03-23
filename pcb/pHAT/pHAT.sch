EESchema Schematic File Version 2
LIBS:pHAT-rescue
LIBS:analog_switches
LIBS:switches
LIBS:pISO
LIBS:pHAT-cache
EELAYER 25 0
EELAYER END
$Descr A 11000 8500
encoding utf-8
Sheet 1 1
Title "pISO"
Date ""
Rev "Rev2"
Comp ""
Comment1 "proto"
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
Text Label 2850 4150 0    60   ~ 0
CS
Text Label 2850 4450 0    60   ~ 0
SCLK
Text Label 2850 4300 0    60   ~ 0
MOSI
Text Label 6300 4100 2    60   ~ 0
CS
Text Label 6300 4200 2    60   ~ 0
RST
Text Label 2850 4000 0    60   ~ 0
RST
Text Label 6300 4300 2    60   ~ 0
DC
Text Label 2850 4600 0    60   ~ 0
DC
Text Label 6300 4600 2    60   ~ 0
SCLK
Text Label 6300 4700 2    60   ~ 0
MOSI
$Comp
L TEST_1P W5
U 1 1 59A9707C
P 1750 3300
F 0 "W5" H 1750 3570 50  0000 C CNN
F 1 "POWER" H 1750 3500 50  0000 C CNN
F 2 "pISO:SLP" H 1950 3300 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-3-15-20-75-14-11-0/ED8183-ND/1147051" H 1950 3300 50  0001 C CNN
F 4 ".30" H 1750 3300 60  0001 C CNN "Price"
	1    1750 3300
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X30 P4
U 1 1 59AC40C4
P 6500 4350
F 0 "P4" H 6500 5900 50  0000 C CNN
F 1 "CONN_01X30" V 6600 4350 50  0000 C CNN
F 2 "pISO:FPC_30" H 6500 4350 50  0001 C CNN
F 3 "http://www.wisechip.com.tw/s/2/product-380039/0-96%E2%80%9D-OLED-Display-UG-2864HLBEG01.html" H 6500 4350 50  0001 C CNN
	1    6500 4350
	1    0    0    -1  
$EndComp
Text Label 6300 4000 2    60   ~ 0
GND
Text Label 6300 3800 2    60   ~ 0
GND
Text Label 5500 3600 2    60   ~ 0
GND
$Comp
L C C1
U 1 1 59AC4E95
P 5850 3000
F 0 "C1" H 5875 3100 50  0000 L CNN
F 1 "2.2uF" H 5875 2900 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 5888 2850 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL21F225ZOFNNNG/1276-6497-1-ND/5958125" H 5850 3000 50  0001 C CNN
F 4 ".02" H 5850 3000 60  0001 C CNN "Price"
	1    5850 3000
	0    -1   -1   0   
$EndComp
$Comp
L C C2
U 1 1 59AC5106
P 5900 3200
F 0 "C2" H 5925 3300 50  0000 L CNN
F 1 "2.2uF" H 5925 3100 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 5938 3050 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL21F225ZOFNNNG/1276-6497-1-ND/5958125" H 5900 3200 50  0001 C CNN
F 4 ".02" H 5900 3200 60  0001 C CNN "Price"
	1    5900 3200
	0    -1   -1   0   
$EndComp
$Comp
L C VCC1
U 1 1 59AC5455
P 5450 5600
F 0 "VCC1" H 5475 5700 50  0000 L CNN
F 1 "10uF" H 5475 5500 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 5488 5450 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL21A106KQCLRNC/1276-2405-1-ND/3890491" H 5450 5600 50  0001 C CNN
F 4 ".03" H 5450 5600 60  0001 C CNN "Price"
	1    5450 5600
	0    -1   -1   0   
$EndComp
$Comp
L C VBAT1
U 1 1 59AC5566
P 5700 3400
F 0 "VBAT1" H 5725 3500 50  0000 L CNN
F 1 "2.2uF" H 5725 3300 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 5738 3250 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL21F225ZOFNNNG/1276-6497-1-ND/5958125" H 5700 3400 50  0001 C CNN
F 4 ".02" H 5700 3400 60  0001 C CNN "Price"
	1    5700 3400
	0    -1   -1   0   
$EndComp
$Comp
L C VDD1
U 1 1 59AC5739
P 5850 3700
F 0 "VDD1" H 5875 3800 50  0000 L CNN
F 1 "2.2uF" H 5875 3600 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 5888 3550 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL21F225ZOFNNNG/1276-6497-1-ND/5958125" H 5850 3700 50  0001 C CNN
F 4 ".02" H 5850 3700 60  0001 C CNN "Price"
	1    5850 3700
	0    -1   -1   0   
$EndComp
$Comp
L +3.3V #PWR01
U 1 1 59AC5DDC
P 6100 2700
F 0 "#PWR01" H 6100 2550 50  0001 C CNN
F 1 "+3.3V" H 6100 2840 50  0000 C CNN
F 2 "" H 6100 2700 50  0000 C CNN
F 3 "" H 6100 2700 50  0000 C CNN
	1    6100 2700
	1    0    0    -1  
$EndComp
Text Label 6300 3900 2    60   ~ 0
GND
NoConn ~ 6300 4400
NoConn ~ 6300 4500
NoConn ~ 6300 4800
NoConn ~ 6300 4900
NoConn ~ 6300 5000
NoConn ~ 6300 5100
NoConn ~ 6300 5200
NoConn ~ 6300 5300
$Comp
L C VCOMH1
U 1 1 59AC605A
P 5750 5500
F 0 "VCOMH1" H 5775 5600 50  0000 L CNN
F 1 "2.2uF" H 5775 5400 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805" H 5788 5350 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/samsung-electro-mechanics/CL21F225ZOFNNNG/1276-6497-1-ND/5958125" H 5750 5500 50  0001 C CNN
F 4 ".02" H 5750 5500 60  0001 C CNN "Price"
	1    5750 5500
	0    -1   -1   0   
$EndComp
Text Label 5050 5600 2    60   ~ 0
GND
$Comp
L R IREF1
U 1 1 59AC6E29
P 6050 5400
F 0 "IREF1" V 6130 5400 50  0000 C CNN
F 1 "390" V 6050 5400 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 5980 5400 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/stackpole-electronics-inc/RMCF0805JT390R/RMCF0805JT390RCT-ND/1942549" H 6050 5400 50  0001 C CNN
F 4 ".006" V 6050 5400 60  0001 C CNN "Price"
	1    6050 5400
	0    1    1    0   
$EndComp
Text Label 6300 2900 2    60   ~ 0
GND
Wire Wire Line
	2850 4150 2750 4150
Wire Wire Line
	2850 4000 2750 4000
Wire Wire Line
	2750 4300 2850 4300
Wire Wire Line
	6000 3000 6300 3000
Wire Wire Line
	6300 3100 5650 3100
Wire Wire Line
	5650 3100 5650 3000
Wire Wire Line
	5650 3000 5700 3000
Wire Wire Line
	6050 3200 6300 3200
Wire Wire Line
	6300 3300 5700 3300
Wire Wire Line
	5700 3300 5700 3200
Wire Wire Line
	5700 3200 5750 3200
Wire Wire Line
	6300 3400 5850 3400
Wire Wire Line
	6000 3700 6300 3700
Wire Wire Line
	5550 3700 5550 3400
Wire Wire Line
	5500 3600 6300 3600
Connection ~ 5550 3600
Wire Wire Line
	6100 2700 6100 3700
Connection ~ 6100 3400
Connection ~ 6100 3700
Wire Wire Line
	6300 5500 5900 5500
Wire Wire Line
	6300 5600 5600 5600
Wire Wire Line
	5600 5500 5200 5500
Wire Wire Line
	5200 5400 5200 5700
Wire Wire Line
	5050 5600 5300 5600
Wire Wire Line
	6300 5800 6300 5700
Wire Wire Line
	6300 5700 5200 5700
Connection ~ 5200 5600
Wire Wire Line
	6300 5400 6200 5400
Wire Wire Line
	5900 5400 5200 5400
Connection ~ 5200 5500
Wire Wire Line
	5700 3700 5550 3700
Wire Wire Line
	4500 2750 4500 2850
Connection ~ 4500 2850
Wire Wire Line
	3750 2400 4600 2400
Wire Wire Line
	4500 2350 4500 2450
Connection ~ 4500 2400
$Comp
L TEST_1P M1
U 1 1 5A09A255
P 2600 5650
F 0 "M1" H 2600 5920 50  0000 C CNN
F 1 "mount1" H 2600 5850 50  0000 C CNN
F 2 "pISO:mount" H 2800 5650 50  0001 C CNN
F 3 "" H 2800 5650 50  0000 C CNN
	1    2600 5650
	1    0    0    -1  
$EndComp
$Comp
L TEST_1P M3
U 1 1 5A09A2EA
P 3000 5750
F 0 "M3" H 3000 6020 50  0000 C CNN
F 1 "mount1" H 3000 5950 50  0000 C CNN
F 2 "pISO:mount" H 3200 5750 50  0001 C CNN
F 3 "" H 3200 5750 50  0000 C CNN
	1    3000 5750
	1    0    0    -1  
$EndComp
$Comp
L TEST_1P M4
U 1 1 5A09A36E
P 3200 5800
F 0 "M4" H 3200 6070 50  0000 C CNN
F 1 "mount1" H 3200 6000 50  0000 C CNN
F 2 "pISO:mount" H 3400 5800 50  0001 C CNN
F 3 "" H 3400 5800 50  0000 C CNN
	1    3200 5800
	1    0    0    -1  
$EndComp
$Comp
L TEST_1P M2
U 1 1 5A09A3F1
P 2800 5700
F 0 "M2" H 2800 5970 50  0000 C CNN
F 1 "mount1" H 2800 5900 50  0000 C CNN
F 2 "pISO:mount" H 3000 5700 50  0001 C CNN
F 3 "" H 3000 5700 50  0000 C CNN
	1    2800 5700
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR02
U 1 1 5A09ABEB
P 2900 5800
F 0 "#PWR02" H 2900 5550 50  0001 C CNN
F 1 "GND" H 2900 5650 50  0000 C CNN
F 2 "" H 2900 5800 50  0000 C CNN
F 3 "" H 2900 5800 50  0000 C CNN
	1    2900 5800
	1    0    0    -1  
$EndComp
Wire Wire Line
	2600 5800 3200 5800
Connection ~ 3000 5800
Connection ~ 2900 5800
Connection ~ 2800 5800
$Comp
L +3.3V #PWR03
U 1 1 5A11071F
P 4500 2350
F 0 "#PWR03" H 4500 2200 50  0001 C CNN
F 1 "+3.3V" H 4500 2490 50  0000 C CNN
F 2 "" H 4500 2350 50  0000 C CNN
F 3 "" H 4500 2350 50  0000 C CNN
	1    4500 2350
	1    0    0    -1  
$EndComp
$Comp
L TEST_1P W13
U 1 1 5A110919
P 4600 2400
F 0 "W13" H 4600 2670 50  0000 C CNN
F 1 "3V3" H 4600 2600 50  0000 C CNN
F 2 "pISO:SLP" H 4800 2400 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 4800 2400 50  0001 C CNN
F 4 ".28" H 4600 2400 60  0001 C CNN "Price"
	1    4600 2400
	0    1    1    0   
$EndComp
$Comp
L TEST_1P W7
U 1 1 5A110B08
P 2750 4000
F 0 "W7" H 2750 4270 50  0000 C CNN
F 1 "RST" H 2750 4200 50  0000 C CNN
F 2 "pISO:SLP" H 2950 4000 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 2950 4000 50  0001 C CNN
F 4 ".28" H 2750 4000 60  0001 C CNN "Price"
	1    2750 4000
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P W8
U 1 1 5A110B8D
P 2750 4150
F 0 "W8" H 2750 4420 50  0000 C CNN
F 1 "CS" H 2750 4350 50  0000 C CNN
F 2 "pISO:SLP" H 2950 4150 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 2950 4150 50  0001 C CNN
F 4 ".28" H 2750 4150 60  0001 C CNN "Price"
	1    2750 4150
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P W9
U 1 1 5A110E5E
P 2750 4300
F 0 "W9" H 2750 4570 50  0000 C CNN
F 1 "MOSI" H 2750 4500 50  0000 C CNN
F 2 "pISO:SLP" H 2950 4300 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 2950 4300 50  0001 C CNN
F 4 ".28" H 2750 4300 60  0001 C CNN "Price"
	1    2750 4300
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P W10
U 1 1 5A110EF5
P 2750 4450
F 0 "W10" H 2750 4720 50  0000 C CNN
F 1 "SCLK" H 2750 4650 50  0000 C CNN
F 2 "pISO:SLP" H 2950 4450 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 2950 4450 50  0001 C CNN
F 4 ".28" H 2750 4450 60  0001 C CNN "Price"
	1    2750 4450
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P W11
U 1 1 5A110F69
P 2750 4600
F 0 "W11" H 2750 4870 50  0000 C CNN
F 1 "DC" H 2750 4800 50  0000 C CNN
F 2 "pISO:SLP" H 2950 4600 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 2950 4600 50  0001 C CNN
F 4 ".28" H 2750 4600 60  0001 C CNN "Price"
	1    2750 4600
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P W12
U 1 1 5A111097
P 2750 4750
F 0 "W12" H 2750 5020 50  0000 C CNN
F 1 "GND" H 2750 4950 50  0000 C CNN
F 2 "pISO:SLP" H 2950 4750 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 2950 4750 50  0001 C CNN
F 4 ".28" H 2750 4750 60  0001 C CNN "Price"
	1    2750 4750
	0    -1   -1   0   
$EndComp
Text Label 2100 2250 2    60   ~ 0
DM1_N
Text Label 2100 2150 2    60   ~ 0
DM1_P
$Comp
L TEST_1P W14
U 1 1 5A114174
P 4050 5650
F 0 "W14" H 4050 5920 50  0000 C CNN
F 1 "D_N" H 4050 5850 50  0000 C CNN
F 2 "pISO:SLP" H 4250 5650 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 4250 5650 50  0001 C CNN
F 4 ".28" H 4050 5650 60  0001 C CNN "Price"
	1    4050 5650
	-1   0    0    1   
$EndComp
$Comp
L TEST_1P W15
U 1 1 5A114230
P 4300 5650
F 0 "W15" H 4300 5920 50  0000 C CNN
F 1 "D_P" H 4300 5850 50  0000 C CNN
F 2 "pISO:SLP" H 4500 5650 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 4500 5650 50  0001 C CNN
F 4 ".28" H 4300 5650 60  0001 C CNN "Price"
	1    4300 5650
	-1   0    0    1   
$EndComp
Text Label 3750 2400 2    60   ~ 0
3V3
Wire Wire Line
	2850 4600 2750 4600
Wire Wire Line
	2750 4450 2850 4450
$Comp
L GND #PWR04
U 1 1 5A117A99
P 5050 5600
F 0 "#PWR04" H 5050 5350 50  0001 C CNN
F 1 "GND" H 5050 5450 50  0000 C CNN
F 2 "" H 5050 5600 50  0000 C CNN
F 3 "" H 5050 5600 50  0000 C CNN
	1    5050 5600
	1    0    0    -1  
$EndComp
Text Label 3850 4200 2    60   ~ 0
3V3
Text Label 4000 4800 2    60   ~ 0
GND
Text Label 4000 4600 2    60   ~ 0
CS
Text Label 5000 4700 0    60   ~ 0
RST
Text Label 4000 4700 2    60   ~ 0
DC
Text Label 5000 4600 0    60   ~ 0
SCLK
Text Label 5000 4800 0    60   ~ 0
MOSI
$Comp
L R R5
U 1 1 5A11B226
P 3850 4350
F 0 "R5" V 3930 4350 50  0000 C CNN
F 1 "10k" V 3850 4350 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 3780 4350 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/stackpole-electronics-inc/RMCF0805JT10K0/RMCF0805JT10K0CT-ND/1942577" H 3850 4350 50  0001 C CNN
F 4 ".006" V 3850 4350 60  0001 C CNN "Price"
	1    3850 4350
	1    0    0    -1  
$EndComp
Text Label 5000 4500 0    60   ~ 0
3V3
$Comp
L TEST_1P NO1
U 1 1 5A11EB0C
P 3700 3450
F 0 "NO1" H 3700 3720 50  0000 C CNN
F 1 "VDD5" H 3700 3650 50  0000 C CNN
F 2 "pISO:SLP" H 3900 3450 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 3900 3450 50  0001 C CNN
F 4 ".28" H 3700 3450 60  0001 C CNN "Price"
	1    3700 3450
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P A1
U 1 1 5A11EBC7
P 4050 3150
F 0 "A1" H 4050 3420 50  0000 C CNN
F 1 "VDD5" H 4050 3350 50  0000 C CNN
F 2 "pISO:SLP" H 4250 3150 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 4250 3150 50  0001 C CNN
F 4 ".28" H 4050 3150 60  0001 C CNN "Price"
	1    4050 3150
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P B1
U 1 1 5A11EC65
P 4500 2850
F 0 "B1" H 4500 3120 50  0000 C CNN
F 1 "VDD5" H 4500 3050 50  0000 C CNN
F 2 "pISO:SLP" H 4700 2850 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 4700 2850 50  0001 C CNN
F 4 ".28" H 4500 2850 60  0001 C CNN "Price"
	1    4500 2850
	0    -1   -1   0   
$EndComp
Wire Wire Line
	2600 5650 2600 5800
Wire Wire Line
	2800 5700 2800 5800
Wire Wire Line
	3000 5750 3000 5800
Wire Wire Line
	4000 4500 3850 4500
$Comp
L ATTINY25/45/85 U1
U 1 1 5A2BAB57
P 4500 4650
F 0 "U1" H 4500 4400 60  0000 C CNN
F 1 "ATTINY25/45/85" H 4500 4900 60  0000 C CNN
F 2 "Housings_SOIC:SOIJ-8_5.3x5.3mm_Pitch1.27mm" H 4500 4600 60  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/microchip-technology/ATTINY25V-10SH/1611-ATTINY25V-10SH-ND/6829774" H 4500 4600 60  0001 C CNN
F 4 ".51" H 4500 4650 60  0001 C CNN "Price"
	1    4500 4650
	1    0    0    -1  
$EndComp
Text Label 4050 5650 1    60   ~ 0
DM1_N
Text Label 4300 5650 1    60   ~ 0
DM1_P
Wire Wire Line
	4100 2450 4100 2400
Connection ~ 4100 2400
Wire Wire Line
	3750 2400 3750 2450
Wire Wire Line
	4100 2750 4100 3150
Wire Wire Line
	3750 2750 3750 3450
NoConn ~ 6300 3500
Text Label 2850 4750 0    60   ~ 0
GND
Wire Wire Line
	2850 4750 2750 4750
Wire Wire Line
	4050 3150 4150 3150
Connection ~ 4100 3150
Wire Wire Line
	3700 3450 3800 3450
Connection ~ 3750 3450
Wire Wire Line
	4200 3450 5050 3450
Wire Wire Line
	5050 3450 5050 2850
Wire Wire Line
	5050 2850 4900 2850
Wire Wire Line
	4550 3150 5050 3150
Connection ~ 5050 3150
Text Label 5050 3150 0    60   ~ 0
GND
$Comp
L SW_Push SW1
U 1 1 5A2C33B8
P 4000 3450
F 0 "SW1" H 4050 3550 50  0000 L CNN
F 1 "SW_Push" H 4000 3390 50  0000 C CNN
F 2 "pISO:spst" H 4000 3650 50  0001 C CNN
F 3 "https://www.digikey.com/products/en/switches/tactile-switches/197?k=&pkeyword=&pv69=3&FV=1f140000%2Cffe000c5&mnonly=0&ColumnSort=1000011&page=1&quantity=750&ptm=0&fid=0&pageSize=500" H 4000 3650 50  0001 C CNN
F 4 ".13" H 4000 3450 60  0001 C CNN "Price"
	1    4000 3450
	1    0    0    -1  
$EndComp
$Comp
L SW_Push SW2
U 1 1 5A2C36C0
P 4350 3150
F 0 "SW2" H 4400 3250 50  0000 L CNN
F 1 "SW_Push" H 4350 3090 50  0000 C CNN
F 2 "pISO:spst" H 4350 3350 50  0001 C CNN
F 3 "https://www.digikey.com/products/en/switches/tactile-switches/197?k=&pkeyword=&pv69=3&FV=1f140000%2Cffe000c5&mnonly=0&ColumnSort=1000011&page=1&quantity=750&ptm=0&fid=0&pageSize=500" H 4350 3350 50  0001 C CNN
F 4 ".13" H 4350 3150 60  0001 C CNN "Price"
	1    4350 3150
	1    0    0    -1  
$EndComp
$Comp
L SW_Push SW3
U 1 1 5A2C3749
P 4700 2850
F 0 "SW3" H 4750 2950 50  0000 L CNN
F 1 "SW_Push" H 4700 2790 50  0000 C CNN
F 2 "pISO:spst" H 4700 3050 50  0001 C CNN
F 3 "https://www.digikey.com/products/en/switches/tactile-switches/197?k=&pkeyword=&pv69=3&FV=1f140000%2Cffe000c5&mnonly=0&ColumnSort=1000011&page=1&quantity=750&ptm=0&fid=0&pageSize=500" H 4700 3050 50  0001 C CNN
F 4 ".13" H 4700 2850 60  0001 C CNN "Price"
	1    4700 2850
	1    0    0    -1  
$EndComp
$Comp
L R R1
U 1 1 5AB435AF
P 3750 2600
F 0 "R1" V 3830 2600 50  0000 C CNN
F 1 "10k" V 3750 2600 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 3680 2600 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/stackpole-electronics-inc/RMCF0805JT10K0/RMCF0805JT10K0CT-ND/1942577" H 3750 2600 50  0001 C CNN
F 4 ".006" V 3750 2600 60  0001 C CNN "Price"
	1    3750 2600
	1    0    0    -1  
$EndComp
$Comp
L R R2
U 1 1 5AB4375B
P 4100 2600
F 0 "R2" V 4180 2600 50  0000 C CNN
F 1 "10k" V 4100 2600 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 4030 2600 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/stackpole-electronics-inc/RMCF0805JT10K0/RMCF0805JT10K0CT-ND/1942577" H 4100 2600 50  0001 C CNN
F 4 ".006" V 4100 2600 60  0001 C CNN "Price"
	1    4100 2600
	1    0    0    -1  
$EndComp
$Comp
L R R3
U 1 1 5AB437FD
P 4500 2600
F 0 "R3" V 4580 2600 50  0000 C CNN
F 1 "10k" V 4500 2600 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 4430 2600 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/stackpole-electronics-inc/RMCF0805JT10K0/RMCF0805JT10K0CT-ND/1942577" H 4500 2600 50  0001 C CNN
F 4 ".006" V 4500 2600 60  0001 C CNN "Price"
	1    4500 2600
	1    0    0    -1  
$EndComp
$Comp
L TEST_1P W2
U 1 1 5AB441ED
P 2050 4100
F 0 "W2" H 2050 4370 50  0000 C CNN
F 1 "ExtraPOGO" H 2050 4300 50  0000 C CNN
F 2 "pISO:SLP" H 2250 4100 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 2250 4100 50  0001 C CNN
F 4 ".28" H 2050 4100 60  0001 C CNN "Price"
	1    2050 4100
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P DNP1
U 1 1 5AB442FA
P 2050 4650
F 0 "DNP1" H 2050 4920 50  0000 C CNN
F 1 "LOGO" H 2050 4850 50  0000 C CNN
F 2 "pISO:LOGO" H 2250 4650 50  0001 C CNN
F 3 "" H 2250 4650 50  0001 C CNN
	1    2050 4650
	0    -1   -1   0   
$EndComp
$Comp
L TEST_1P W1
U 1 1 5AB445A1
P 1800 4200
F 0 "W1" H 1800 4470 50  0000 C CNN
F 1 "ExtraPOGO" H 1800 4400 50  0000 C CNN
F 2 "pISO:SLP" H 2000 4200 50  0001 C CNN
F 3 "https://www.digikey.com/product-detail/en/mill-max-manufacturing-corp/0906-0-15-20-76-14-11-0/ED8180-ND/1147048" H 2000 4200 50  0001 C CNN
F 4 ".28" H 1800 4200 60  0001 C CNN "Price"
	1    1800 4200
	0    -1   -1   0   
$EndComp
$Comp
L USBC U2
U 1 1 5AB44745
P 2550 2200
F 0 "U2" H 2550 1550 60  0000 C CNN
F 1 "USBC" H 2550 2850 60  0000 C CNN
F 2 "pISO:USBC" H 2550 2000 60  0001 C CNN
F 3 "" H 2550 2000 60  0001 C CNN
	1    2550 2200
	1    0    0    -1  
$EndComp
Text Label 2100 1650 2    60   ~ 0
GND
Text Label 2950 1650 0    60   ~ 0
GND
Text Label 2950 2750 0    60   ~ 0
GND
Text Label 2100 2750 2    60   ~ 0
GND
Text Label 1750 3300 0    60   ~ 0
piPOWER
Text Label 2100 1950 2    60   ~ 0
piPOWER
Text Label 2100 2450 2    60   ~ 0
piPOWER
Text Label 2950 1950 0    60   ~ 0
piPOWER
Text Label 2950 2450 0    60   ~ 0
piPOWER
NoConn ~ 2100 1750
NoConn ~ 2100 1850
NoConn ~ 2100 2050
NoConn ~ 2100 2350
NoConn ~ 2100 2550
NoConn ~ 2100 2650
NoConn ~ 2950 2550
NoConn ~ 2950 2650
NoConn ~ 2950 2350
NoConn ~ 2950 2050
NoConn ~ 2950 1850
NoConn ~ 2950 1750
Text Label 2950 2150 0    60   ~ 0
DM1_P
Text Label 2950 2250 0    60   ~ 0
DM1_N
$EndSCHEMATC
