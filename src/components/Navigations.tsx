import { useState } from 'react';
import { makeStyles } from '@mui/styles';
import Box from '@mui/material/Box';
import BottomNavigation from '@mui/material/BottomNavigation';
import BottomNavigationAction from '@mui/material/BottomNavigationAction';
import RestoreIcon from '@mui/icons-material/Restore';
import FavoriteIcon from '@mui/icons-material/Favorite';
import LocationOnIcon from '@mui/icons-material/LocationOn';

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

const Navigations = () => {
    const classes = useStyles();
    const [value, setValue] = useState(0);
    return (
        <Box className={classes.root}>
            <BottomNavigation
                showLabels
                value={value}
                onChange={(event, newValue) => {
                    setValue(newValue);
                }}
            >
                <BottomNavigationAction
                    label='Send'
                    icon={<RestoreIcon />}
                    className={classes.navAction}
                />
                <BottomNavigationAction
                    label='Receive'
                    icon={<FavoriteIcon />}
                    className={classes.navAction}
                />
                <BottomNavigationAction
                    label='Explorer'
                    icon={<LocationOnIcon />}
                    className={classes.navAction}
                />
            </BottomNavigation>
        </Box>
    );
};

export default Navigations;
