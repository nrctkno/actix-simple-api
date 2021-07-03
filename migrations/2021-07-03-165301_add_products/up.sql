CREATE TABLE `products` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(50) NOT NULL,
  `price` decimal(12,2) NULL,
  `stock` int(11) NOT NULL,
  `status` enum('Available','NotAvailable') NOT NULL,
   PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

INSERT INTO `products` (`id`, `name`, `price`, `stock`, `status`) VALUES
(1, 'Yandy jacket', 764, 35, 'NotAvailable'),
(2, 'Jim & Jago skirt', 425.05, 5, 'Available'),
(3, 'A’reve overall', 348.35, 66, 'NotAvailable'),
(4, 'English Factory leggings', 864.56, 98, 'Available'),
(5, 'Glyder sweatpants', 245.71, 100, 'NotAvailable'),
(6, 'Hendrix sweater / jumper', 203.59, 17, 'NotAvailable'),
(7, 'Aqua 4 Swimwear mittens', 719.69, 96, 'NotAvailable'),
(8, 'Pas De Deux gloves', 469.19, 78, 'Available'),
(9, 'Hunny Bunny Baby shoes', 651.5, 12, 'Available'),
(10, 'Legacy scarf', 182.02, 100, 'NotAvailable'),
(11, 'Tobi hat', 838.97, 12, 'NotAvailable'),
(12, 'Heartloom raincoat', 140.74, 94, 'NotAvailable'),
(13, 'Sunglow Fashion T-shirt', 451.26, 11, 'Available'),
(14, 'Bee Free overall', 546.01, 67, 'NotAvailable'),
(15, 'Bishop + Young belt', 166.48, 18, 'NotAvailable'),
(16, 'Spotlight on Style shawl', 270.7, 29, 'Available'),
(17, 'Chantelle skirt', 118.29, 11, 'Available'),
(18, 'Porcelain Apparel tuxedo', 347.26, 48, 'Available'),
(19, 'Zara overall', 774.71, 89, 'NotAvailable'),
(20, 'Mouse Creek Trading Co. underpants', 640.8, 99, 'NotAvailable'),
(21, 'Apparel 360 swimming trunks', 16.88, 39, 'NotAvailable'),
(22, 'Luna Boutique underpants', 847.83, 37, 'Available'),
(23, 'After Market jeans', 963.47, 82, 'Available'),
(24, 'Yandy swimming trunks', 580.71, 74, 'NotAvailable'),
(25, 'Hunny Bunny Baby wedding dress', 251.37, 70, 'NotAvailable'),
(26, 'Heartloom polo shirt', 808.63, 58, 'Available'),
(27, 'Apparel 360 jacket', 650.88, 84, 'NotAvailable'),
(28, 'For all wedding dress', 688.99, 31, 'NotAvailable'),
(29, 'Spotlight on Style suit', 437.76, 11, 'NotAvailable'),
(30, 'Foreign Falcon mittens', 529.73, 82, 'NotAvailable'),
(31, 'Gilt polo shirt', 642.65, 67, 'NotAvailable'),
(32, 'English Factory wedding dress', 402.47, 37, 'Available'),
(33, 'Shein swimsuit', 856.34, 79, 'Available'),
(34, 'Spotlight on Style socks', 149.58, 91, 'Available'),
(35, 'Bailey 44 scarf', 451.86, 99, 'Available'),
(36, 'Classy Missy jeans', 821.08, 83, 'Available'),
(37, 'Pumpkin Clothing Co. pantyhose', 813.63, 69, 'Available'),
(38, 'Chris & Carol shoes', 891.3, 24, 'Available'),
(39, 'Heartloom tracksuit', 410.01, 5, 'Available'),
(40, 'Chris & Carol pullover', 222.58, 88, 'NotAvailable'),
(41, 'Hunny Bunny Baby slacks', 36.11, 36, 'Available'),
(42, 'Marika leggings', 566.01, 29, 'Available'),
(43, 'Rosebuds for Girls hat', 306.73, 61, 'Available'),
(44, 'Fahrenheit bikini', 725.59, 20, 'NotAvailable'),
(45, 'Rosy Cheeks Children’s Apparel tuxedo', 875.23, 46, 'Available'),
(46, 'Luna Boutique shirt', 856.09, 70, 'Available'),
(47, 'Heartloom wedding dress', 300.51, 80, 'Available'),
(48, 'Bailey 44 shorts', 868.13, 78, 'NotAvailable'),
(49, 'A’reve dress', 843.82, 97, 'Available'),
(50, 'Hunny Bunny Baby bra', 844.54, 71, 'Available'),
(51, 'Apparel 360 overall', 856.89, 7, 'NotAvailable'),
(52, 'Jelly Kelly raincoat', 745.02, 21, 'Available'),
(53, 'Foreign Falcon dress', 88.37, 58, 'NotAvailable'),
(54, 'Sunglow Fashion belt', 301.8, 75, 'Available'),
(55, 'Bailey 44 jacket', 652.21, 74, 'NotAvailable'),
(56, 'Hendrix cargo pants', 738.35, 35, 'NotAvailable'),
(57, 'Yandy hat', 108.84, 10, 'NotAvailable'),
(58, 'Porcelain Apparel cargo pants', 176.44, 39, 'NotAvailable'),
(59, 'Hunny Bunny Baby suit', 771.63, 57, 'Available'),
(60, 'Uniqlo jacket', 82.69, 87, 'Available'),
(61, 'Children’s Wear sweatshirt', 728.78, 30, 'Available'),
(62, 'Bee Free skirt', 213.12, 3, 'Available'),
(63, 'Tobi gloves', 529.25, 98, 'NotAvailable'),
(64, 'Urban Touch trousers', 510.08, 71, 'NotAvailable'),
(65, 'Marika sweatpants', 123.02, 1, 'Available'),
(66, 'Orchid cardigan', 188.21, 31, 'NotAvailable'),
(67, 'Marika leggings', 975.27, 0, 'NotAvailable'),
(68, 'Nike trench coat', 938.62, 21, 'NotAvailable'),
(69, 'Fahrenheit cargo pants', 297.99, 9, 'Available'),
(70, 'Children’s Wear shoes', 32.31, 87, 'NotAvailable'),
(71, 'Bishop + Young bra', 801, 34, 'Available'),
(72, 'Legacy trench coat', 904.17, 4, 'Available'),
(73, 'Aqua 4 Swimwear trench coat', 960.83, 21, 'Available'),
(74, 'Tobi jeans', 737.85, 60, 'Available'),
(75, 'Studio 51 Clothing Co. suit', 647.01, 75, 'Available'),
(76, 'Pumpkin Clothing Co. tank top', 325.03, 81, 'NotAvailable'),
(77, 'Children’s Wear cardigan', 478.4, 10, 'NotAvailable'),
(78, 'Modern Walk socks', 750.31, 54, 'Available'),
(79, 'After Market vest', 32.07, 91, 'Available'),
(80, 'Tanked Up Co. cap', 137.17, 12, 'NotAvailable'),
(81, 'Aqua 4 Swimwear socks', 755.5, 73, 'Available'),
(82, 'Tater Tots trench coat', 123.19, 50, 'NotAvailable'),
(83, 'Legacy cargo pants', 991.83, 16, 'NotAvailable'),
(84, 'Heartloom hoodie', 69.79, 7, 'Available'),
(85, 'Hunny Bunny Baby coat', 672.72, 17, 'Available'),
(86, 'Modern Walk shorts', 748.24, 27, 'NotAvailable'),
(87, 'Yandy shoes', 295.72, 47, 'NotAvailable'),
(88, 'Bishop + Young tuxedo', 900.49, 48, 'NotAvailable'),
(89, 'Mystree cargo pants', 810.15, 12, 'Available'),
(90, 'Nike pantyhose', 696.22, 96, 'NotAvailable'),
(91, 'Foreign Falcon slacks', 358.65, 89, 'Available'),
(92, 'Adventure Apparel Co. trench coat', 423.57, 22, 'NotAvailable'),
(93, 'Koral blouse', 850.85, 37, 'NotAvailable'),
(94, 'Sunglow Fashion cap', 357.21, 70, 'NotAvailable'),
(95, 'Aqua 4 Swimwear polo shirt', 670.23, 34, 'NotAvailable'),
(96, 'Asos bikini', 326.26, 91, 'NotAvailable'),
(97, 'Nike cardigan', 790.52, 47, 'Available'),
(98, 'Heartloom sweater / jumper', 334.43, 96, 'Available'),
(99, 'Coco + Carmen tank top', 169.39, 41, 'Available')
;
