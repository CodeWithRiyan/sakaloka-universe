-- Bill of Materials (BOM) Schema
-- Supports F&B/Restaurant and Clothing/Manufacturing

-- BOM: Parent assembly linking to a product
CREATE TABLE boms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id),
    version VARCHAR(20) DEFAULT '1.0.0',
    status VARCHAR(20) DEFAULT 'draft' CHECK (status IN ('draft', 'active', 'archived')),
    effective_from TIMESTAMPTZ,
    effective_to TIMESTAMPTZ,
    total_cost BIGINT, -- in smallest currency unit
    notes TEXT,
    created_by UUID,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    CONSTRAINT uq_bom_product_version UNIQUE (product_id, version)
);

CREATE INDEX idx_boms_org ON boms(organization_id);
CREATE INDEX idx_boms_product ON boms(product_id);
CREATE INDEX idx_boms_status ON boms(status);

-- BOM Items: Components in a BOM
CREATE TABLE bom_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bom_id UUID NOT NULL REFERENCES boms(id) ON DELETE CASCADE,
    component_product_id UUID NOT NULL REFERENCES products(id),
    quantity DECIMAL(12,4) NOT NULL,
    unit VARCHAR(20) NOT NULL, -- g, ml, pcs, meters, tbsp, etc.
    waste_percent DECIMAL(5,2) DEFAULT 0,
    yield_percent DECIMAL(5,2) DEFAULT 100,
    unit_cost BIGINT, -- snapshot cost in smallest currency unit
    line_cost BIGINT, -- calculated: qty × unit_cost
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    CONSTRAINT uq_bom_item_component UNIQUE (bom_id, component_product_id)
);

CREATE INDEX idx_bom_items_bom ON bom_items(bom_id);
CREATE INDEX idx_bom_items_product ON bom_items(component_product_id);

-- Production Runs: Track actual production
CREATE TABLE bom_production_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bom_id UUID NOT NULL REFERENCES boms(id),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    quantity_produced DECIMAL(12,4) NOT NULL,
    estimated_cost BIGINT,
    actual_cost BIGINT,
    status VARCHAR(20) DEFAULT 'planned' CHECK (status IN ('planned', 'in_progress', 'completed', 'cancelled')),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_by UUID,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_production_runs_org ON bom_production_runs(organization_id);
CREATE INDEX idx_production_runs_bom ON bom_production_runs(bom_id);
CREATE INDEX idx_production_runs_status ON bom_production_runs(status);

-- BOM Consumption: Track actual consumption per production run
CREATE TABLE bom_consumption (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    production_run_id UUID NOT NULL REFERENCES bom_production_runs(id) ON DELETE CASCADE,
    component_product_id UUID NOT NULL REFERENCES products(id),
    planned_quantity DECIMAL(12,4) NOT NULL,
    actual_quantity DECIMAL(12,4) NOT NULL,
    unit VARCHAR(20) NOT NULL,
    waste_quantity DECIMAL(12,4) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_bom_consumption_run ON bom_consumption(production_run_id);
CREATE INDEX idx_bom_consumption_product ON bom_consumption(component_product_id);

-- Function to auto-update updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_boms_updated_at
    BEFORE UPDATE ON boms
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bom_items_updated_at
    BEFORE UPDATE ON bom_items
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bom_production_runs_updated_at
    BEFORE UPDATE ON bom_production_runs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();