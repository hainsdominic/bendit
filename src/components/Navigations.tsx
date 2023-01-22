import { useState } from 'react';
import { makeStyles } from '@mui/styles';
import BottomNavigation from '@mui/material/BottomNavigation';
import BottomNavigationAction from '@mui/material/BottomNavigationAction';
import SendIcon from '@mui/icons-material/Send';
import CallReceivedIcon from '@mui/icons-material/CallReceived';
import ExploreIcon from '@mui/icons-material/Explore';

const useStyles = makeStyles({
    root: {
        position: 'fixed',
        bottom: 0,
        width: '100%',
    },
    navAction: {
        flexGrow: 1,
    },
});

//types for the props
type Props = {
    value: number;
    setValue: (value: number) => void;
};

const Navigations = ({ value, setValue }: Props) => {
    const classes = useStyles();

    return (
        <div className={classes.root}>
            <BottomNavigation
                showLabels
                value={value}
                onChange={(event, newValue) => {
                    setValue(newValue);
                }}
            >
                <BottomNavigationAction
                    label='Send'
                    icon={<SendIcon />}
                    className={classes.navAction}
                />
                <BottomNavigationAction
                    label='Receive'
                    icon={<CallReceivedIcon />}
                    className={classes.navAction}
                />
                <BottomNavigationAction
                    label='Explorer'
                    icon={<ExploreIcon />}
                    className={classes.navAction}
                />
            </BottomNavigation>
        </div>
    );
};

export default Navigations;
