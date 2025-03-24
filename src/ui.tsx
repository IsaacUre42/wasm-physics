import React, {useEffect, useState} from 'react';
import {Box, ColorInput, Slider, Switch, Text, Tabs, Container} from "@mantine/core";
import {BallParams, BoxParams} from "./types.ts";

type AppProps = {
    setBallParams: (params: BallParams) => void,
    setBoxParams: (params: BoxParams) => void,
    setActiveTab: (tab: string) => void,
}

const Ui: React.FC<AppProps> = ({setBallParams, setBoxParams, setActiveTab}) => {
    let [size, setSize] = useState(15);
    let [ballColor, setBallColor] = useState("#00FFFF");
    let [ballRestitution, setBallRestitution] = useState(0.5);
    let [mass, setMass] = useState(3);
    let [fixed, setFixed] = useState(false);

    let [boxWidth, setBoxWidth] = useState(100);
    let [boxHeight, setBoxHeight] = useState(100);
    let [boxRestitution, setBoxRestitution] = useState(1.0);
    let [boxColor, setBoxColor] = useState("#000000");



    useEffect(() => {
        let params = {
            size: size,
            color: ballColor,
            restitution: ballRestitution,
            mass: mass,
            fixed: fixed
        };

        setBallParams(params);
    }, [size, ballColor, ballRestitution, mass, fixed]);

    useEffect(() => {
        let params = {
            color: boxColor,
            width: boxWidth,
            height: boxHeight,
            restitution: boxRestitution
        };

        setBoxParams(params);
    }, [boxColor, boxWidth, boxHeight, boxRestitution]);

    return (
        <Box>
            <Tabs defaultValue={"ball"} onChange={value => value ? setActiveTab(value) : setActiveTab("ball")}>
                <Tabs.List>
                    <Tabs.Tab value={"ball"}>
                        Ball
                    </Tabs.Tab>
                    <Tabs.Tab value={"box"}>
                        Box
                    </Tabs.Tab>
                    <Tabs.Tab value={"fun"}>
                        Fun
                    </Tabs.Tab>
                </Tabs.List>

                <Tabs.Panel value={"ball"}>
                    <Container>
                        <Switch
                            label="Static"
                            onChange={value => setFixed(value.currentTarget.checked)}
                            style={{marginTop: "10px"}}
                        />
                        <Text>Size</Text>
                        <Slider
                            defaultValue={15}
                            min={1}
                            max={50}
                            label={(value)=> value.toFixed(0)}
                            step={1}
                            styles={{markLabel: {display: 'none'}}}
                            onChangeEnd={setSize}
                        />
                        <Text>Mass</Text>
                        <Slider
                            defaultValue={12}
                            min={1}
                            max={50}
                            label={(value)=> value.toFixed(0)}
                            step={1}
                            styles={{markLabel: {display: 'none'}}}
                            onChangeEnd={value => setMass(value)}
                        />
                        <Text>Bounciness</Text>
                        <Slider
                            defaultValue={ballRestitution}
                            min={0}
                            max={2}
                            label={(value)=> value.toFixed(1)}
                            step={0.1}
                            styles={{markLabel: {display: 'none'}}}
                            onChangeEnd={setBallRestitution}
                        />
                        <Text>Colour</Text>
                        <ColorInput
                            format={"hex"}
                            onChangeEnd={setBallColor}
                            disallowInput
                            defaultValue={ballColor}
                        />
                    </Container>
                </Tabs.Panel>

                <Tabs.Panel value={"box"}>
                    <Container>
                        <Text>Width</Text>
                        <Slider
                            defaultValue={boxWidth}
                            min={1}
                            max={500}
                            label={(value)=> value.toFixed(0)}
                            step={1}
                            styles={{markLabel: {display: 'none'}}}
                            onChangeEnd={setBoxWidth}
                        />
                        <Text>Height</Text>
                        <Slider
                            defaultValue={boxHeight}
                            min={1}
                            max={500}
                            label={(value)=> value.toFixed(0)}
                            step={1}
                            styles={{markLabel: {display: 'none'}}}
                            onChangeEnd={setBoxHeight}
                        />
                        <Text>Bounciness</Text>
                        <Slider
                            defaultValue={boxRestitution}
                            min={0}
                            max={2}
                            label={(value)=> value.toFixed(1)}
                            step={0.1}
                            styles={{markLabel: {display: 'none'}}}
                            onChangeEnd={setBoxRestitution}
                        />
                        <Text>Colour</Text>
                        <ColorInput
                            format={"hex"}
                            onChangeEnd={setBoxColor}
                            disallowInput
                            defaultValue={boxColor}
                        />
                    </Container>
                </Tabs.Panel>

                <Tabs.Panel value={"fun"}>
                    <Container>
                        <Text>:)</Text>
                    </Container>
                </Tabs.Panel>

            </Tabs>
        </Box>
    );
};

export default Ui;