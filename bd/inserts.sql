INSERT INTO vehiculo
VALUES 
('CH123456789', 'ABC-123', 'Modelo X', 'Marca A', 'Rojo', 2020, '2021-05-30', 6720, 'DISPONIBLE'),
('CH987654321', 'XYZ-456', 'Modelo Y', 'Marca B', 'Azul', 2021, '2022-07-12', 4265, 'DISPONIBLE'),
('CH192837465', 'LMN-789', 'Modelo Z', 'Marca C', 'Negro', 2022, '2023-01-03', 3200, 'DISPONIBLE'),
('CH564738291', 'JKL-012', 'Modelo W', 'Marca D', 'Blanco', 2019, '2020-09-06', 12900, 'DISPONIBLE'),
('CH102938475', 'OPQ-345', 'Modelo V', 'Marca E', 'Verde', 2023, '2024-05-19', 22700, 'DISPONIBLE'),
('BER123456690','PER-185','Modelo X', 'Marca A', 'Verde', 2020, '2020-09-06', 5300, 'DISPONIBLE'),
('LOW578948226', 'HOW-349','Modelo X','Marca A', 'Verde', 2020, '2020-09-06', 5300, 'DISPONIBLE'),
('GUW837832048','GLO-098','Modelo X', 'Marca A', 'Verde', 2020, '2020-09-06', 5300, 'DISPONIBLE'),
('UYS123535245','ERM-234','Modelo X','Marca A', 'Verde', 2020, '2020-09-06', 5300, 'DISPONIBLE');

INSERT INTO vendedor ("nombre", "apellido", "cedula")
VALUES('Jose Andres', 'Guido Escobar', '001-140405-100J'),
('Marcelo', 'Mendoza', '001-291197-009K'),
('Guillermo Ernesto', 'Castillo', '001-060996-1025B');

INSERT INTO cliente("nombre","apellido", "cedula")
VALUES('Carlos', 'Garcia', '201-041187-044X'),
('Gabriel', 'Marquez', '005-280175-032H'),
('Peter', 'Romero', '001-090499-3456D');

INSERT INTO gasto ("matricula", "tipo_reparacion", "monto", "fecha_finalizacion", "nombre_taller", "direccion_taller", "telefono_taller") 
VALUES ('XYZ-456','Cambio de filtro de aceite','800','03-02-2024','Rapidito','De la perfecta 100 mts al sur','2222-4444'),
 ('LMN-789','Llantas','5000','03-10-2024','Rapidito','De la perfecta 100 mts al sur','2222-4444');

INSERT INTO promesa_venta("id_cliente","matricula","monto_promesa","estado", "fecha_realizacion","fecha_finalizacion")
VALUES(1,'LMN-789',800,'RESERVADO', '2024-10-28', '2025-02-02'),
(2 ,'JKL-012', 300, 'RESERVADO', '2024-10-27', '2025-02-01');

UPDATE vehiculo
SET estado = 'DISPONIBLE'
WHERE matricula = 'LMN-789';


INSERT INTO venta ("matricula", "fecha_venta", "precio_venta", "id_cliente","id_vendedor")
VALUES('XYZ-456','2024-10-29', 7835, 3, 2);


INSERT INTO Cliente (nombre, apellido, cedula) VALUES
('Juan', 'Pérez', '1234567890'),
('María', 'González', '2345678901'),
('Carlos', 'Sánchez', '3456789012'),
('Ana', 'Martínez', '4567890123'),
('Luis', 'López', '5678901234'),
('Isabel', 'Hernández', '6789012345'),
('Javier', 'Ramírez', '7890123456'),
('Laura', 'Torres', '8901234567'),
('Diego', 'Flores', '9012345678'),
('Claudia', 'Gutiérrez', '0123456789'),
('Fernando', 'Vargas', '1234567891'),
('Valentina', 'Castro', '2345678902'),
('Miguel', 'Rojas', '3456789013'),
('Sofía', 'Salazar', '4567890124'),
('Andrés', 'Mendoza', '5678901235'),
('Carmen', 'Ponce', '6789012346'),
('Nicolás', 'Bravo', '7890123457'),
('Carolina', 'Marín', '8901234568'),
('Esteban', 'Acosta', '9012345679'),
('Luisa', 'Bermúdez', '0123456790'),
('Pablo', 'Soto', '1234567892'),
('Mónica', 'Cordero', '2345678903'),
('Ricardo', 'Valdés', '3456789014'),
('Teresa', 'Peña', '4567890125'),
('Gabriel', 'Hidalgo', '5678901236'),
('Patricia', 'Alonso', '6789012347'),
('Julio', 'Zapata', '7890123458'),
('Ana', 'Jiménez', '8901234569'),
('Roberto', 'Lara', '9012345680'),
('Santiago', 'Díaz', '0123456791'),
('Vanessa', 'Rincón', '1234567893');

INSERT INTO gasto ("matricula", "tipo_reparacion", "monto", "fecha_finalizacion", "nombre_taller", "direccion_taller", "telefono_taller")
VALUES ('HOW-349','Cambio de filtro de aceite','800','03-02-2024','Rapidito','De la perfecta 100 mts al sur','2222-4444'),
 ('GLO-098','Llantas','5000','03-10-2024','Rapidito','De la perfecta 100 mts al sur','2222-4444');

 INSERT INTO promesa_venta("id_cliente","matricula","monto_promesa","estado", "fecha_realizacion","fecha_finalizacion")
VALUES(1,'PER-185',800,'RESERVADO', '2024-10-28', '2025-02-02'),
(2 ,'ERM-234', 300, 'RESERVADO', '2024-10-27', '2025-02-01');

INSERT INTO venta ("matricula", "fecha_venta", "precio_venta", "id_cliente","id_vendedor")
VALUES('HOW-349','2024-10-29', 7835, 3, 3);


INSERT INTO Vehiculo (nro_chasis, matricula, modelo, marca, color, anio, fecha_compra, precio_compra, estado) VALUES
('CH00123456789012345', 'ABC001', 'Model S', 'Tesla', 'Rojo', 2022, '2022-03-15', 79999.99, 'DISPONIBLE'),
('CH00123456789012346', 'ABC002', 'Civic', 'Honda', 'Azul', 2021, '2021-05-10', 22999.50, 'ENPROCESO'),
('CH00123456789012347', 'ABC003', 'Corolla', 'Toyota', 'Negro', 2020, '2020-07-20', 19999.75, 'VENDIDO'),
('CH00123456789012348', 'ABC004', 'Model 3', 'Tesla', 'Blanco', 2023, '2023-01-25', 59999.00, 'RESERVADO'),
('CH00123456789012349', 'ABC005', 'Focus', 'Ford', 'Plateado', 2019, '2019-11-30', 17999.99, 'DISPONIBLE'),
('CH00123456789012350', 'ABC006', 'Mustang', 'Ford', 'Amarillo', 2022, '2022-04-10', 55999.00, 'ENPROCESO'),
('CH00123456789012351', 'ABC007', 'A4', 'Audi', 'Gris', 2020, '2020-06-18', 39999.00, 'VENDIDO'),
('CH00123456789012352', 'ABC008', 'Accord', 'Honda', 'Azul', 2021, '2021-08-25', 26999.99, 'RESERVADO'),
('CH00123456789012353', 'ABC009', 'Camry', 'Toyota', 'Negro', 2021, '2021-02-15', 24999.50, 'DISPONIBLE'),
('CH00123456789012354', 'ABC010', 'Impala', 'Chevrolet', 'Blanco', 2020, '2020-05-30', 22999.75, 'ENPROCESO'),
('CH00123456789012355', 'ABC011', 'Altima', 'Nissan', 'Rojo', 2023, '2023-03-05', 27999.99, 'VENDIDO'),
('CH00123456789012356', 'ABC012', 'Cherokee', 'Jeep', 'Verde', 2019, '2019-12-20', 29999.00, 'RESERVADO'),
('CH00123456789012357', 'ABC013', 'Outback', 'Subaru', 'Marrón', 2021, '2021-09-10', 31999.50, 'DISPONIBLE'),
('CH00123456789012358', 'ABC014', 'Malibu', 'Chevrolet', 'Plateado', 2022, '2022-02-14', 24999.99, 'ENPROCESO'),
('CH00123456789012359', 'ABC015', 'Crosstrek', 'Subaru', 'Naranja', 2020, '2020-07-28', 23999.00, 'VENDIDO'),
('CH00123456789012360', 'ABC016', '3 Series', 'BMW', 'Azul', 2021, '2021-11-20', 45999.00, 'RESERVADO'),
('CH00123456789012361', 'ABC017', 'X5', 'BMW', 'Negro', 2022, '2022-10-10', 69999.99, 'DISPONIBLE'),
('CH00123456789012362', 'ABC018', 'GLC', 'Mercedes', 'Blanco', 2021, '2021-07-07', 59999.00, 'ENPROCESO'),
('CH00123456789012363', 'ABC019', 'Q5', 'Audi', 'Gris', 2020, '2020-08-30', 49999.99, 'VENDIDO'),
('CH00123456789012364', 'ABC020', 'Rogue', 'Nissan', 'Plateado', 2022, '2022-01-18', 29999.50, 'RESERVADO'),
('CH00123456789012365', 'ABC021', 'F-150', 'Ford', 'Rojo', 2023, '2023-04-05', 48999.00, 'DISPONIBLE'),
('CH00123456789012366', 'ABC022', 'Ram 1500', 'Ram', 'Verde', 2022, '2022-03-22', 45999.99, 'ENPROCESO'),
('CH00123456789012367', 'ABC023', 'Silverado', 'Chevrolet', 'Negro', 2021, '2021-12-15', 47999.00, 'VENDIDO'),
('CH00123456789012368', 'ABC024', 'Tacoma', 'Toyota', 'Azul', 2022, '2022-09-29', 33999.50, 'RESERVADO'),
('CH00123456789012369', 'ABC025', 'Ranger', 'Ford', 'Blanco', 2020, '2020-04-05', 29999.00, 'DISPONIBLE'),
('CH00123456789012370', 'ABC026', 'Canyon', 'GMC', 'Gris', 2021, '2021-06-15', 31999.99, 'ENPROCESO'),
('CH00123456789012371', 'ABC027', 'Sierra', 'GMC', 'Rojo', 2022, '2022-05-25', 54999.00, 'VENDIDO'),
('CH00123456789012372', 'ABC028', 'Forte', 'Kia', 'Plateado', 2021, '2021-03-15', 22999.50, 'RESERVADO'),
('CH00123456789012373', 'ABC029', 'Sonata', 'Hyundai', 'Negro', 2020, '2020-08-05', 24999.75, 'DISPONIBLE'),
('CH00123456789012374', 'ABC030', 'Elantra', 'Hyundai', 'Blanco', 2023, '2023-02-20', 19999.99, 'ENPROCESO');


INSERT INTO Venta (matricula, fecha_venta, precio_venta, id_cliente, id_vendedor) VALUES
('ABC003', '2022-04-01', 19999.75, 1, 1),
('ABC011', '2023-04-10', 27999.99, 2, 2),
('ABC015', '2023-05-15', 23999.00, 3, 3),
('ABC019', '2023-06-20', 49999.99, 4, 3),
('ABC023', '2023-07-25', 47999.00, 5, 2),
('ABC027', '2023-08-30', 54999.00, 6, 1);

INSERT INTO Promesa_Venta (id_cliente, matricula, monto_promesa, estado, fecha_realizacion, fecha_finalizacion) VALUES
(1, 'ABC004', 59999.00, 'RESERVADO', '2023-01-25', NULL),
(2, 'ABC008', 26999.99, 'RESERVADO', '2021-08-25', NULL),
(3, 'ABC012', 29999.00, 'RESERVADO', '2019-12-20', NULL),                       
(4, 'ABC016', 45999.00, 'RESERVADO', '2021-11-20', NULL),
(5, 'ABC020', 29999.50, 'RESERVADO', '2022-01-18', NULL),
(6, 'ABC024', 33999.50, 'RESERVADO', '2022-09-29', NULL),
(7, 'ABC028', 22999.50, 'RESERVADO', '2021-03-15', NULL);

INSERT INTO Gasto (matricula, tipo_reparacion, monto, fecha_finalizacion, nombre_taller, direccion_taller, telefono_taller) VALUES
('ABC004', 'Cambio de aceite', 150.00, '2023-02-01', 'Taller El Buen Manejo', 'Calle 123, Ciudad', '555-1234'),
('ABC008', 'Reemplazo de frenos', 400.00, '2021-09-15', 'Frenos Rápidos', 'Avenida 45, Ciudad', '555-5678'),
('ABC012', 'Ajuste de suspensión', 350.00, '2020-01-10', 'Suspensiones Pro', 'Calle 789, Ciudad', '555-9101'),
('ABC016', 'Reparación de motor', 1200.00, '2021-12-05', 'Motores y Más', 'Avenida 12, Ciudad', '555-1112'),
('ABC020', 'Mantenimiento general', 200.00, '2022-02-20', 'Taller Multiservicio', 'Calle 34, Ciudad', '555-1314'),
('ABC024', 'Cambio de batería', 300.00, '2022-10-10', 'Baterías y Talleres', 'Calle 56, Ciudad', '555-1516'),
('ABC028', 'Reparación de transmisión', 800.00, '2021-04-15', 'Transmisiones del Norte', 'Avenida 78, Ciudad', '555-1718');

INSERT INTO "public"."gasto" ("matricula", "tipo_reparacion", "monto", "fecha_finalizacion", "nombre_taller", "direccion_taller", "telefono_taller")
VALUES ('ABC003','Cambio de aceite','700','2021-06-16','Rapiditos','Avenida esta, 32','555-1342'),
('ABC011','Cambio de vidrio delatero','2000','2022-07-29','La casa del vidrio','Testa avenida 45','555-0987'),
('ABC015','Overhaull','5893','2022-03-01','El chino','Managua calle 12','555-3214'),
('ABC019','Mantenimiento de frenos','1000','2020-10-05','Brembo Ni','Ticuantepe avenida 5','555-3134');