CREATE TABLE Vehiculo (
    nro_chasis VARCHAR(50) UNIQUE NOT NULL,
    matricula VARCHAR(15) PRIMARY KEY,
    modelo VARCHAR(50) NOT NULL,
    marca VARCHAR(50) NOT NULL,
    color VARCHAR(30) NOT NULL,
    anio INT NOT NULL,
    fecha_compra DATE NOT NULL,
    precio_compra NUMERIC(12,2) NOT NULL
);


-- CREATE TABLE Compra (
--     id_compra SERIAL PRIMARY KEY,
--     fecha_compra DATE NOT NULL,
--     precio_compra NUMERIC(12, 2) NOT NULL,
--     gastos_viaje NUMERIC(12, 2) NOT NULL,
--     matricula VARCHAR(15) NOT NULL,
--     FOREIGN KEY (matricula) REFERENCES Vehiculo (matricula)
-- );


CREATE TABLE Gasto (
    id_gasto SERIAL PRIMARY KEY,
    matricula VARCHAR(15) NOT NULL,
    tipo_reparacion VARCHAR(100) NOT NULL,
    monto NUMERIC(12, 2) NOT NULL,
    fecha_finalizacion DATE NOT NULL,
    nombre_taller VARCHAR(50) NOT NULL,
    direccion_taller VARCHAR(100),
    telefono_taller VARCHAR(20),
    FOREIGN KEY (matricula) REFERENCES Vehiculo (matricula)
);


CREATE TABLE Cliente (
    id_cliente SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    apellido VARCHAR(50) NOT NULL,
    cedula VARCHAR(16) UNIQUE NOT NULL
);


CREATE TABLE Vendedor (
    id_vendedor SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    apellido VARCHAR(50) NOT NULL,
    cedula VARCHAR(16) UNIQUE NOT NULL
);


CREATE TABLE Venta (
    id_venta SERIAL PRIMARY KEY,
    matricula VARCHAR(15) NOT NULL,
    fecha_venta DATE NOT NULL,
    precio_venta NUMERIC(12, 2) NOT NULL,
    id_cliente INT NOT NULL,
    id_vendedor INT NOT NULL,
    FOREIGN KEY (matricula) REFERENCES Vehiculo (matricula),
    FOREIGN KEY (id_cliente) REFERENCES Cliente (id_cliente),
    FOREIGN KEY (id_vendedor) REFERENCES Vendedor (id_vendedor)
);


CREATE TABLE Promesa_Venta (
    id_promesa SERIAL PRIMARY KEY,
    id_cliente INT NOT NULL,
    matricula VARCHAR(15) NOT NULL,
    monto_promesa NUMERIC(12, 2) NOT NULL,
    estado VARCHAR(50) NOT NULL,
    fecha_realizacion DATE NOT NULL,
    fecha_finalizacion DATE,
    FOREIGN KEY (id_cliente) REFERENCES Cliente (id_cliente),
    FOREIGN KEY (matricula) REFERENCES Vehiculo (matricula)
);

CREATE TYPE state AS ENUM('DISPONIBLE', 'ENPROCESO', 'VENDIDO', 'RESERVADO');

ALTER TABLE vehiculo ADD COLUMN estado state;


INSERT INTO vehiculo
VALUES
('CH123456789', 'ABC-123', 'Modelo X', 'Marca A', 'Rojo', 2020, '2021-05-30', 6720),
('CH987654321', 'XYZ-456', 'Modelo Y', 'Marca B', 'Azul', 2021, '2022-07-12', 4265),
('CH192837465', 'LMN-789', 'Modelo Z', 'Marca C', 'Negro', 2022, '2023-01-03', 3200),
('CH564738291', 'JKL-012', 'Modelo W', 'Marca D', 'Blanco', 2019, '2020-09-06', 12900),
('CH102938475', 'OPQ-345', 'Modelo V', 'Marca E', 'Verde', 2023, '2024-05-19', 22700);

INSERT INTO vehiculo
VALUES
('BER123456690','PER-185','Modelo X', 'Marca A', 'Verde', 2020, '2020-09-06', 5300, 'DISPONIBLE'),
('LOW578948226', 'HOW-349','Modelo X','Marca A', 'Verde', 2020, '2020-09-06', 5300, 'DISPONIBLE'),
('GUW837832048','GLO-098','Modelo X', 'Marca A', 'Verde', 2020, '2020-09-06', 5300, 'DISPONIBLE'),
('UYS123535245','ERM-234','Modelo X','Marca A', 'Verde', 2020, '2020-09-06', 5300, 'DISPONIBLE');
UPDATE vehiculo
SET estado='DISPONIBLE'
WHERE (vehiculo.matricula = 'OPQ-345');


select * FROM vehiculo;

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


-- INSERT INTO compra("fecha_compra", "precio_compra","gastos_viaje","matricula")
-- -- fecha mes/dia/anio
-- -- correcion el formato es YY-MM-DD
-- VALUES('02-01-2020','5000','200','XYZ-456'),
-- ('10-02-2020', '6000', '400', 'LMN-789'),
-- ('03-23-2020', '4000', '360', 'JKL-012'),
-- ('12-04-2020', '4567', '1206','OPQ-345'),
-- ('07-24-2020', '3742', '752', 'ABC-123');


INSERT INTO promesa_venta("id_cliente","matricula","monto_promesa","estado", "fecha_realizacion","fecha_finalizacion")
VALUES(1,'LMN-789',800,'RESERVADO', '2024-10-28', '2025-02-02'),
(2 ,'JKL-012', 300, 'RESERVADO', '2024-10-27', '2025-02-01');

UPDATE vehiculo
SET estado = 'DISPONIBLE'
WHERE matricula = 'LMN-789';



INSERT INTO venta ("matricula", "fecha_venta", "precio_venta", "id_cliente","id_vendedor")
VALUES('XYZ-456','2024-10-29', 7835, 3, 2);


SELECT * FROM vehiculo;

-- SELECT vehiculo.*, compra.precio_compra FROM vehiculo
-- JOIN compra on compra.matricula=vehiculo.matricula;

SELECT * FROM vehiculo
WHERE fecha_compra BETWEEN '2020-01-02' AND '2020-10-02';

SELECT nro_chasis, precio_compra FROM vehiculo
WHERE fecha_compra BETWEEN '2020-01-02' AND '2020-10-02';

SELECT *FROM venta
JOIN cliente ON cliente.id_cliente = venta.id_cliente
JOIN vehiculo ON vehiculo.matricula = venta.matricula
WHERE venta.precio_venta < 8000;

SELECT cliente.nombre, cliente.apellido, vehiculo.matricula FROM cliente
JOIN venta ON venta.id_cliente = cliente.id_cliente
JOIN vehiculo ON vehiculo.matricula = venta.matricula
WHERE venta.precio_venta <8000;

-- SELECT * FROM venta;

-- SELECT * from vehiculo
-- WHERE  DATEPART('year', fecha_compra)=2020 --fecha_compra>='2020-01-02'
-- AND  DATEPART('month', fecha_compra)=2; --fecha_compra <'2020-10-30';
-- --vehiculos disponibles
-- SELECT * FROM vehiculo
-- JOIN compra on vehiculo.matricula=compra.matricula
-- WHERE vehiculo.estado='DISPONIBLE';


--consulta promesas activas y sus respectivos clientes
SELECT cliente.id_cliente, cliente.nombre, cliente.apellido, promesa_venta.matricula, promesa_venta.fecha_realizacion
FROM cliente
JOIN promesa_venta ON cliente.id_cliente = promesa_venta.id_cliente
WHERE promesa_venta.estado = 'RESERVADO';


--costo total de cada vehiculo incluyendo gastos
SELECT vehiculo.matricula, vehiculo.modelo, vehiculo.marca,
(vehiculo.precio_compra + COALESCE(SUM(gasto.monto), 0)) AS costo_total_adquisicion
FROM vehiculo
JOIN gasto on vehiculo.matricula = gasto.matricula
GROUP BY vehiculo.matricula, vehiculo.modelo, vehiculo.precio_compra;

-- ventas realizadas por cada vendedor y el monto total de ellas
SELECT vendedor.id_vendedor, vendedor.nombre, vendedor.apellido,
COUNT(venta.id_venta) AS total_ventas,
SUM(venta.precio_venta) AS monto_total
FROM vendedor
JOIN venta ON vendedor.id_vendedor = venta.id_vendedor
GROUP BY vendedor.id_vendedor, vendedor.nombre, vendedor.apellido;


--Historial completo

SELECT vehiculo.matricula, vehiculo.modelo, vehiculo.marca,
cliente.nombre AS nombre_cliente, cliente.apellido AS apellido_cliente,
vendedor.nombre AS nombre_vendedor, vendedor.apellido AS apellido_vendedor,
venta.fecha_venta, venta.precio_venta
FROM vehiculo
JOIN venta ON vehiculo.matricula = venta.matricula
JOIN cliente ON venta.id_cliente = cliente.id_cliente
JOIN vendedor ON venta.id_vendedor = vendedor.id_vendedor
WHERE vehiculo.estado = 'VENDIDO';

--vehiculos que tienen mas de un gasto
SELECT vehiculo.matricula, vehiculo.modelo, vehiculo.marca,
COUNT(gasto.id_gasto) AS total_gastos,
SUM(gasto.monto) AS monto_total
FROM vehiculo
JOIN gasto ON vehiculo.matricula = gasto.matricula
GROUP BY vehiculo.matricula, vehiculo.modelo, vehiculo.marca
HAVING COUNT(gasto.id_gasto)>1;



-- TRUNCATE promesa_venta CASCADE;


DROP TABLE vehiculo, compra, promesa_venta, gasto, cliente, vendedor, venta;

