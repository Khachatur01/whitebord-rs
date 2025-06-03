use standard_svg_plugin::property_map::PropertyMap;
use standard_svg_plugin::svg_element::svg_circle::SVGCircle;
use standard_svg_plugin::svg_element::svg_line::SVGLine;
use standard_svg_plugin::svg_element::svg_rectangle::SVGRectangle;
use web_sys::{CssStyleDeclaration, SvgCircleElement, SvgElement, SvgLineElement, SvgRectElement};

trait Attributes {
    fn set_attributes(&self, attributes: &PropertyMap);
}
trait Styles {
    fn set_css(&self, css: &PropertyMap);
}

impl Attributes for SvgElement {
    fn set_attributes(&self, attributes: &PropertyMap) {
        for (name, value) in attributes {
            self.set_attribute(name, value).expect(format!("Can't set attribute {name}").as_str());
        }
    }
}

impl Styles for SvgElement {
    fn set_css(&self, css: &PropertyMap) {
        let style: CssStyleDeclaration = self.style();

        for (name, value) in css {
            style.set_property(name, value).expect(format!("Can't set css property {name}").as_str());
        }

        let style = style.css_text();
        self.set_attribute("style", style.as_str()).expect(format!("Can't set css text {style}").as_str());
    }
}

pub trait UpdateSVG<SVGElement> {
    fn update_svg(&mut self, svg_element: &SVGElement, attributes: &PropertyMap, css: &PropertyMap);
}

impl UpdateSVG<SVGCircle> for SvgCircleElement {
    fn update_svg(&mut self, svg_element: &SVGCircle, attributes: &PropertyMap, css: &PropertyMap) {
        self.set_attribute("cx", &svg_element.cx.to_string()).expect("Can't set cx");
        self.set_attribute("cy", &svg_element.cy.to_string()).expect("Can't set cy");
        self.set_attribute("r", &svg_element.r.to_string()).expect("Can't set r");

        self.set_attributes(attributes);
        self.set_css(css);
    }
}

impl UpdateSVG<SVGRectangle> for SvgRectElement {
    fn update_svg(&mut self, svg_element: &SVGRectangle, attributes: &PropertyMap, css: &PropertyMap) {
        self.set_attribute("x", &svg_element.x.to_string()).expect("Can't set x");
        self.set_attribute("y", &svg_element.y.to_string()).expect("Can't set y");
        self.set_attribute("width", &svg_element.width.to_string()).expect("Can't set width");
        self.set_attribute("height", &svg_element.height.to_string()).expect("Can't set height");

        self.set_attributes(attributes);
        self.set_css(css);
    }
}

impl UpdateSVG<SVGLine> for SvgLineElement {
    fn update_svg(&mut self, svg_element: &SVGLine, attributes: &PropertyMap, css: &PropertyMap) {
        self.set_attribute("x1", &svg_element.x1.to_string()).expect("Can't set x1");
        self.set_attribute("y1", &svg_element.y1.to_string()).expect("Can't set y1");
        self.set_attribute("x2", &svg_element.x2.to_string()).expect("Can't set x2");
        self.set_attribute("y2", &svg_element.y2.to_string()).expect("Can't set y2");

        self.set_attributes(attributes);
        self.set_css(css);
    }
}

/* todo: Implement UpdateSVG for all supported svg element types */
