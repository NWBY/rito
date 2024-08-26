<?php

class User
{
    public $username;
    public $email;
    public $sign_in_count;
    public $active;

    function __construct($username, $email, $sign_in_count, $active)
    {
        $this->username = $username;
        $this->email = $email;
        $this->sign_in_count = $sign_in_count;
        $this->active = $active;
    }

    function __toString()
    {
        return $this->username;
    }
}
