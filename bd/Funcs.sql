    -- $$ delimitadores de una cadena de texto
    CREATE OR REPLACE FUNCTION actualizar_estado()
    RETURNS TRIGGER AS $$
    BEGIN
        UPDATE vehiculo
        SET estado = NEW.estado::state -- cambiamos el valor para que este se pueda guardar
        WHERE matricula = NEW.matricula;
        
        RETURN NEW;
    END;
    $$ LANGUAGE plpgsql;
    
    CREATE TRIGGER trigger_actualizar
    AFTER INSERT OR UPDATE OF estado ON promesa_venta
    FOR EACH ROW
    EXECUTE FUNCTION actualizar_estado();
    
    CREATE OR REPLACE FUNCTION estado_vendido()
    RETURNS TRIGGER AS $$
    BEGIN
        UPDATE vehiculo
        SET estado = 'VENDIDO'
        WHERE matricula = NEW.matricula;
        
        RETURN NEW;
    END;
    $$ LANGUAGE plpgsql;
    
    CREATE TRIGGER trigger_venta
    AFTER INSERT ON venta       
    FOR EACH ROW
    EXECUTE FUNCTION estado_vendido();
    
    
    
    -- CREATE OR REPLACE FUNCTION estado_mantenimiento 
    -- RETURNS TRIGGER AS $$
    -- BEGIN
    --     UPDATE vehiculo
    --     SET estado = 'ENPROCESO'
    --     WHERE matricula = NEW.matricula;
        
    --     RETURN NEW;
    -- END;
    -- $$ LANGUAGE plpgsql;
    
    
    DROP TRIGGER IF EXISTS trigger_actualizar ON promesa_venta;
    DROP FUNCTION IF EXISTS actualizar_estado;
    
