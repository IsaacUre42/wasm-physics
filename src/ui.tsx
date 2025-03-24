import React, {useEffect, useState} from 'react';
import {Box, ColorPicker, NumberInput, Slider, Switch, Text} from "@mantine/core";
import {BallParams} from "./types.ts";

type AppProps = {
    setParams: (params: BallParams) => void,
}

const Ui: React.FC<AppProps> = ({setParams}) => {
    let [size, setSize] = useState(5);
    let [color, setColor] = useState("#00FFFF");
    let [restitution, setRestitution] = useState(0.5);
    let [mass, setMass] = useState(3);
    let [fixed, setFixed] = useState(false);
    let [showPreview, setShowPreview] = useState(false);
    let [sizeValue, setSizeValue] = useState(5);

    let preview: HTMLCanvasElement = document.getElementById("preview");



    useEffect(() => {
        let params = {
            size: size,
            color: color,
            restitution: restitution,
            mass: mass,
            fixed: fixed
        };

        setParams(params);
    }, [size, color, restitution, mass, fixed]);

    useEffect(() => {
        let context = preview.getContext("2d");

        if (context) {
            context.clearRect(0, 0, preview.width, preview.height);
            context.beginPath();
            context.arc(preview.width / 2, preview.height / 2, size, 0, 2 * Math.PI);
            context.fillStyle = color;
            context.fill();
        }
    }, [showPreview]);

    useEffect(() => {
        if (showPreview) {
            preview.style.display = "block";
            preview.style.width = "100px";
            preview.style.height = "100px";
            preview.style.top = "50%";
            preview.style.left = "50%";
            preview.style.transform = "translate(-50%, -50%)";
            console.log("show");
        } else {
            preview.style.display = "hidden";
            console.log("hidden");
        }
    }, [showPreview]);

    return (
        <Box>
            <Switch
                label="Static"
                onChange={value => setFixed(value.currentTarget.checked)}
            />
            <Text>Size</Text>
            <Slider
                defaultValue={5}
                value={sizeValue}
                onChange={value => {setSizeValue(value); setShowPreview(true)}}
                min={1}
                max={50}
                label={(value)=> value.toFixed(1)}
                step={1}
                styles={{markLabel: {display: 'none'}}}
                onChangeEnd={value => {setSize(value); setShowPreview(false)}}
            >
            </Slider>
            <Text>Mass</Text>
            <Slider
                defaultValue={3}
                min={1}
                max={50}
                label={(value)=> value.toFixed(1)}
                step={1}
                styles={{markLabel: {display: 'none'}}}
                onChangeEnd={value => setMass(value)}
            >
            </Slider>
            <Text>Restitution</Text>
            <NumberInput
                hideControls
                onChange={value=>setRestitution(Number(value))}
            />
            <Text>Colour</Text>
            <ColorPicker
                format={"hex"}
                onChange={value=> setColor(value)}
            />
        </Box>
    );
};

export default Ui;